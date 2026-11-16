//! The undo history of a text input, kept in its tree state.
//!
//! Each step is the change an edit made: where it replaced text, the text
//! it removed and the text it inserted, with the caret before and after.
//! Undo applies the inverse change and redo applies it again.
use std::collections::VecDeque;
use std::hash::{DefaultHasher, Hash, Hasher};

use super::cursor::Cursor;
use super::value::Value;

/// What an edit did, to decide whether it joins the step before it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    /// A typed character: a run of them is one step.
    Typing,
    /// A character deleted by Backspace or Delete: a run of them is one step.
    Deleting,
    /// A paste, a cut, a word or line deletion: always a step of its own.
    Other,
}

/// The text an edit replaced, at the byte offset `at` of the value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Change {
    pub(crate) at: usize,
    pub(crate) removed: String,
    pub(crate) inserted: String,
}

impl Change {
    /// Apply the change to `value`.
    fn apply(&self, value: &mut Value) {
        value.replace(self.at..self.at + self.removed.len(), &self.inserted);
    }

    /// Apply the inverse of the change to `value`.
    fn revert(&self, value: &mut Value) {
        value.replace(self.at..self.at + self.inserted.len(), &self.removed);
    }

    /// Extend this change with `next`, made right after it at the caret it
    /// left. Returns whether the two are one contiguous change.
    fn join(&mut self, next: Change) -> bool {
        let end = self.at + self.inserted.len();

        if next.removed.is_empty() && next.at == end {
            // Typing on.
            self.inserted.push_str(&next.inserted);
        } else if next.inserted.is_empty() && self.inserted.is_empty() {
            if next.at + next.removed.len() == self.at {
                // Backspace on.
                self.removed.insert_str(0, &next.removed);
                self.at = next.at;
            } else if next.at == self.at {
                // Delete on.
                self.removed.push_str(&next.removed);
            } else {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone)]
struct Step {
    change: Change,
    before: Cursor,
    after: Cursor,
}

/// The steps to undo and to redo.
#[derive(Debug, Clone, Default)]
pub(crate) struct History {
    undo: VecDeque<Step>,
    redo: Vec<Step>,
    /// The kind of the last edit and the caret it left, while the next edit
    /// may join its step: one of the same kind at the same caret.
    open: Option<(Kind, Cursor)>,
    /// The kind of the edit being made, the caret before it, and whether it
    /// may join the open step.
    pending: Option<(Kind, Cursor, bool)>,
    /// A fingerprint of the app's value when the input last saw it, for a
    /// controlled input.
    synced: Option<u64>,
    /// A fingerprint of the text the input last made.
    made: Option<u64>,
}

impl History {
    /// Record that an edit of `kind` is about to be made with the caret at
    /// `cursor`.
    pub(crate) fn record(&mut self, kind: Kind, cursor: Cursor) {
        let joins = kind != Kind::Other && self.open == Some((kind, cursor));
        self.pending = Some((kind, cursor, joins));
        self.open = None;
    }

    /// The recorded edit made `change` to the text, now `value`, and left
    /// the caret at `cursor`. An edit that changed nothing is no step.
    pub(crate) fn edited(&mut self, change: Option<Change>, value: &Value, cursor: Cursor) {
        let Some((kind, before, joins)) = self.pending.take() else {
            return;
        };
        self.made = Some(fingerprint(value));

        let Some(change) = change.filter(|change| change.removed != change.inserted) else {
            return;
        };
        self.redo.clear();

        let joined = joins
            && self.undo.back_mut().is_some_and(|step| {
                let joined = step.change.join(change.clone());
                if joined {
                    step.after = cursor;
                }
                joined
            });
        if !joined {
            self.undo.push_back(Step {
                change,
                before,
                after: cursor,
            });
            if self.undo.len() > LIMIT {
                let _ = self.undo.pop_front();
            }
        }

        self.open = (kind != Kind::Other).then_some((kind, cursor));
    }

    /// The caret is now at `cursor`: a move away from where the last edit
    /// left it ends that edit's step.
    pub(crate) fn follow(&mut self, cursor: Cursor) {
        if self.open.is_some_and(|(_, at)| at != cursor) {
            self.open = None;
        }
    }

    /// See the app's `value` of a controlled input. A value the input did
    /// not make replaced the text from outside, so the steps no longer apply
    /// and are forgotten. A value seen before is stale: iced can rebuild the
    /// view before the app handles an edit.
    pub(crate) fn sync(&mut self, value: &Value) {
        let seen = fingerprint(value);
        if self.synced == Some(seen) {
            return;
        }
        if self.synced.is_some() && self.made != Some(seen) {
            self.undo.clear();
            self.redo.clear();
            self.open = None;
        }
        self.synced = Some(seen);
    }

    /// Undo the last step of `value`, giving the caret before it.
    pub(crate) fn undo(&mut self, value: &mut Value) -> Option<Cursor> {
        let step = self.undo.pop_back()?;
        step.change.revert(value);
        let before = step.before;
        self.redo.push(step);
        self.open = None;
        self.made = Some(fingerprint(value));
        Some(before)
    }

    /// Redo the last undone step of `value`, giving the caret after it.
    pub(crate) fn redo(&mut self, value: &mut Value) -> Option<Cursor> {
        let step = self.redo.pop()?;
        step.change.apply(value);
        let after = step.after;
        self.undo.push_back(step);
        self.open = None;
        self.made = Some(fingerprint(value));
        Some(after)
    }

    pub(crate) fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub(crate) fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
}

/// Tells a value apart from another without keeping a copy of it.
fn fingerprint(value: &Value) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.as_str().hash(&mut hasher);
    hasher.finish()
}

/// The most steps an input keeps. Each holds only the text its edit
/// replaced, so the cap is Chromium's.
const LIMIT: usize = 1000;

#[cfg(test)]
mod tests {
    use super::{History, Kind};
    use crate::text_input::Cursor;
    use crate::text_input::editor::Editor;
    use crate::text_input::value::Value;

    #[test]
    fn a_thousand_steps_undo_and_redo_in_order() {
        let mut value = Value::new("");
        let mut cursor = Cursor::default();
        let mut history = History::default();
        let mut texts = vec![value.to_string()];

        for n in 0..1200 {
            // A caret move between words closes each step.
            history.follow(Cursor::default());
            history.record(Kind::Typing, cursor);
            let change =
                Editor::new(&mut value, &mut cursor).insert(char::from(b'a' + (n % 26) as u8));
            history.edited(change, &value, cursor);
            history.follow(cursor);
            texts.push(value.to_string());
        }

        let mut undone = 0;
        while let Some(at) = history.undo(&mut value) {
            undone += 1;
            cursor = at;
            assert_eq!(value.to_string(), texts[texts.len() - 1 - undone]);
        }
        assert_eq!(undone, super::LIMIT);
        assert_eq!(cursor.end(&value), value.len());

        while history.redo(&mut value).is_some() {}
        assert_eq!(value.to_string(), texts[texts.len() - 1]);
    }
}
