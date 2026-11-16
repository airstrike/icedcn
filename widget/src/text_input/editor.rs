use crate::text_input::history::Change;
use crate::text_input::{Cursor, Value};

/// Edits a [`Value`] at its [`Cursor`]. Each edit returns the change it
/// made, if any, for the undo history.
pub struct Editor<'a> {
    value: &'a mut Value,
    cursor: &'a mut Cursor,
}

impl<'a> Editor<'a> {
    pub fn new(value: &'a mut Value, cursor: &'a mut Cursor) -> Editor<'a> {
        Editor { value, cursor }
    }

    /// The graphemes from `start` to `end`, about to be replaced.
    fn change(&self, start: usize, end: usize) -> Change {
        let offsets = self.value.offsets();
        let (from, to) = (offsets[start], offsets[end]);

        Change {
            at: from,
            removed: self.value.as_str()[from..to].to_owned(),
            inserted: String::new(),
        }
    }

    pub fn insert(&mut self, character: char) -> Option<Change> {
        let mut change = match self.cursor.selection(self.value) {
            Some((left, right)) => {
                let change = self.change(left, right);
                self.cursor.move_left(self.value);
                self.value.remove_many(left, right);
                change
            }
            None => self.change(self.cursor.end(self.value), self.cursor.end(self.value)),
        };

        self.value.insert(self.cursor.end(self.value), character);
        self.cursor.move_right(self.value);

        change.inserted.push(character);
        Some(change)
    }

    pub fn paste(&mut self, content: Value) -> Option<Change> {
        let length = content.len();
        let mut change = match self.cursor.selection(self.value) {
            Some((left, right)) => {
                let change = self.change(left, right);
                self.cursor.move_left(self.value);
                self.value.remove_many(left, right);
                change
            }
            None => self.change(self.cursor.end(self.value), self.cursor.end(self.value)),
        };
        change.inserted.push_str(content.as_str());

        self.value.insert_many(self.cursor.end(self.value), content);

        self.cursor.move_right_by_amount(self.value, length);
        Some(change)
    }

    pub fn backspace(&mut self) -> Option<Change> {
        match self.cursor.selection(self.value) {
            Some((start, end)) => {
                let change = self.change(start, end);
                self.cursor.move_left(self.value);
                self.value.remove_many(start, end);
                Some(change)
            }
            None => {
                let start = self.cursor.start(self.value);

                (start > 0).then(|| {
                    let change = self.change(start - 1, start);
                    self.cursor.move_left(self.value);
                    self.value.remove(start - 1);
                    change
                })
            }
        }
    }

    pub fn delete(&mut self) -> Option<Change> {
        match self.cursor.selection(self.value) {
            Some(_) => self.backspace(),
            None => {
                let end = self.cursor.end(self.value);

                (end < self.value.len()).then(|| {
                    let change = self.change(end, end + 1);
                    self.value.remove(end);
                    change
                })
            }
        }
    }
}
