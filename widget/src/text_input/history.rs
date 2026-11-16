//! The undo history of a text input, kept in its tree state.
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

/// The text and caret before each step, to undo it, and after each undone
/// step, to redo it.
#[derive(Debug, Clone, Default)]
pub(crate) struct History {
    undo: Vec<(Value, Cursor)>,
    redo: Vec<(Value, Cursor)>,
    /// The kind of the last edit and the caret it left, while the next edit
    /// may join its step: one of the same kind at the same caret.
    open: Option<(Kind, Cursor)>,
    /// The kind of the edit being made.
    pending: Option<Kind>,
    /// The edit being made pushed a step.
    pushed: bool,
    /// The app's value when the input last saw it, for a controlled input.
    synced: Option<Value>,
    /// The text the input last made.
    made: Option<Value>,
}

impl History {
    /// Record that an edit of `kind` is about to change `value`, with the
    /// caret at `cursor`.
    pub(crate) fn record(&mut self, kind: Kind, value: &Value, cursor: Cursor) {
        self.pushed = kind == Kind::Other || self.open != Some((kind, cursor));
        if self.pushed {
            self.undo.push((value.clone(), cursor));
            if self.undo.len() > LIMIT {
                let _ = self.undo.remove(0);
            }
        }
        self.open = None;
        self.pending = (kind != Kind::Other).then_some(kind);
    }

    /// The recorded edit left `value` with the caret at `cursor`. An edit
    /// that changed nothing is no step.
    pub(crate) fn edited(&mut self, value: &Value, cursor: Cursor) {
        if std::mem::take(&mut self.pushed)
            && self.undo.last().is_some_and(|(before, _)| before == value)
        {
            let _ = self.undo.pop();
            self.pending = None;
        } else {
            self.redo.clear();
        }
        self.open = self.pending.take().map(|kind| (kind, cursor));
        self.made = Some(value.clone());
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
        if self.synced.as_ref() == Some(value) {
            return;
        }
        if self.synced.is_some() && self.made.as_ref() != Some(value) {
            self.undo.clear();
            self.redo.clear();
            self.open = None;
        }
        self.synced = Some(value.clone());
    }

    /// Undo the last step of the text now `value` with the caret at
    /// `cursor`, giving the text and caret before it.
    pub(crate) fn undo(&mut self, value: &Value, cursor: Cursor) -> Option<(Value, Cursor)> {
        let before = self.undo.pop()?;
        self.redo.push((value.clone(), cursor));
        self.open = None;
        self.made = Some(before.0.clone());
        Some(before)
    }

    /// Redo the last undone step, giving the text and caret after it.
    pub(crate) fn redo(&mut self, value: &Value, cursor: Cursor) -> Option<(Value, Cursor)> {
        let after = self.redo.pop()?;
        self.undo.push((value.clone(), cursor));
        self.open = None;
        self.made = Some(after.0.clone());
        Some(after)
    }

    pub(crate) fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub(crate) fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
}

/// The most steps an input keeps.
const LIMIT: usize = 100;
