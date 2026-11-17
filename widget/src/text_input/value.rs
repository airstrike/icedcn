use unicode_segmentation::UnicodeSegmentation;

use crate::edit::{self, Platform};

/// The value of a [`TextInput`].
///
/// The text is kept in one string with the byte offset where each grapheme
/// starts, so a copy is two allocations however long the text is.
///
/// [`TextInput`]: super::TextInput
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    text: String,
    /// The byte offset of every grapheme's start, then the text's length.
    bounds: Vec<usize>,
}

impl Value {
    /// Creates a new [`Value`] from a string slice.
    pub fn new(string: &str) -> Self {
        Self {
            text: string.to_owned(),
            bounds: segment(string),
        }
    }

    /// Returns whether the [`Value`] is empty or not.
    ///
    /// A [`Value`] is empty when it contains no graphemes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the total amount of graphemes in the [`Value`].
    pub fn len(&self) -> usize {
        self.bounds.len() - 1
    }

    /// Returns the text of the [`Value`].
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// The byte offset of every grapheme's start, then the text's length.
    pub(crate) fn offsets(&self) -> &[usize] {
        &self.bounds
    }

    /// The byte offset of the grapheme at `index`, or the text's length.
    fn offset(&self, index: usize) -> usize {
        self.bounds[index.min(self.len())]
    }

    /// The grapheme index of the byte `offset`, which starts a grapheme or
    /// ends the text.
    fn index(&self, offset: usize) -> usize {
        self.bounds.partition_point(|&start| start < offset)
    }

    /// Returns the position of the previous start of a word from the given
    /// grapheme `index`.
    pub fn previous_start_of_word(&self, index: usize) -> usize {
        let index = index.min(self.len());
        let previous_string = &self.text[..self.offset(index)];

        UnicodeSegmentation::split_word_bound_indices(previous_string)
            .rfind(|(_, word)| !word.trim_start().is_empty())
            .map(|(i, _)| self.index(i))
            .unwrap_or(0)
    }

    /// Returns the position of the next end of a word from the given grapheme
    /// `index`.
    pub fn next_end_of_word(&self, index: usize) -> usize {
        let start = self.offset(index);
        let next_string = &self.text[start..];

        UnicodeSegmentation::split_word_bound_indices(next_string)
            .find(|(_, word)| !word.trim_start().is_empty())
            .map(|(i, next_word)| self.index(start + i + next_word.len()))
            .unwrap_or(self.len())
    }

    /// Where a word motion to the left from grapheme `index` stops on
    /// `platform`.
    pub(crate) fn word_left(&self, index: usize, platform: Platform) -> usize {
        self.index(edit::word_left(&self.text, self.offset(index), platform))
    }

    /// Where a word motion to the right from grapheme `index` stops on
    /// `platform`.
    pub(crate) fn word_right(&self, index: usize, platform: Platform) -> usize {
        self.index(edit::word_right(&self.text, self.offset(index), platform))
    }

    /// Returns the position of the start of the next word after the given
    /// grapheme `index`, past the spaces after the word at `index`.
    pub fn next_start_of_word(&self, index: usize) -> usize {
        let next = crate::edit::next_word_start(&self.text, self.offset(index));
        self.index(next)
    }

    /// Returns a new [`Value`] containing the graphemes from `start` until the
    /// given `end`.
    pub fn select(&self, start: usize, end: usize) -> Self {
        let start = start.min(self.len());
        let end = end.clamp(start, self.len());
        let from = self.bounds[start];

        Self {
            text: self.text[from..self.bounds[end]].to_owned(),
            bounds: self.bounds[start..=end].iter().map(|b| b - from).collect(),
        }
    }

    /// Returns a new [`Value`] containing the graphemes until the given
    /// `index`.
    pub fn until(&self, index: usize) -> Self {
        self.select(0, index)
    }

    /// Inserts a new `char` at the given grapheme `index`.
    ///
    /// The `char` may join the graphemes around it, as a combining mark
    /// does.
    pub fn insert(&mut self, index: usize, c: char) {
        let index = index.min(self.len());
        let at = self.bounds[index];
        let added = c.len_utf8();
        self.text.insert(at, c);

        // Breaking graphemes restarts at any boundary: segment again from the
        // grapheme before the char until a boundary after it meets an old one.
        let from = index.saturating_sub(1);
        let start = self.bounds[from];
        let end = self.text.len() - start;
        let mut fresh = Vec::new();
        let mut old = index;
        let mut tail = self.bounds.len() - 1;

        for (i, _) in UnicodeSegmentation::grapheme_indices(&self.text[start..], true)
            .chain(std::iter::once((end, "")))
        {
            let bound = start + i;
            if bound > at {
                old += self.bounds[old..]
                    .iter()
                    .take_while(|&&b| b + added < bound)
                    .count();
                if self.bounds[old] + added == bound {
                    tail = old;
                    break;
                }
            }
            fresh.push(bound);
        }

        for bound in &mut self.bounds[tail..] {
            *bound += added;
        }
        let _ = self.bounds.splice(from..tail, fresh);
    }

    /// Inserts a bunch of graphemes at the given grapheme `index`.
    pub fn insert_many(&mut self, index: usize, value: Value) {
        let index = index.min(self.len());
        let at = self.bounds[index];
        let added = value.text.len();

        self.text.insert_str(at, &value.text);
        for bound in &mut self.bounds[index + 1..] {
            *bound += added;
        }
        let _ = self.bounds.splice(
            index + 1..index + 1,
            value.bounds[1..].iter().map(|b| b + at),
        );
    }

    /// Removes the grapheme at the given `index`.
    pub fn remove(&mut self, index: usize) {
        self.remove_many(index, index + 1);
    }

    /// Removes the graphemes from `start` to `end`.
    pub fn remove_many(&mut self, start: usize, end: usize) {
        let end = end.min(self.len());
        let start = start.min(end);
        let (from, to) = (self.bounds[start], self.bounds[end]);

        let _ = self.text.drain(from..to);
        let _ = self.bounds.drain(start + 1..=end);
        for bound in &mut self.bounds[start + 1..] {
            *bound -= to - from;
        }
    }

    /// Replaces the bytes in `range`, which start and end graphemes, with
    /// `text`.
    pub(crate) fn replace(&mut self, range: std::ops::Range<usize>, text: &str) {
        self.text.replace_range(range, text);
        self.bounds = segment(&self.text);
    }

    /// Returns a new [`Value`] with all its graphemes replaced with the
    /// dot ('•') character.
    pub fn secure(&self) -> Self {
        let dot = '•'.len_utf8();

        Self {
            text: "•".repeat(self.len()),
            bounds: (0..=self.len()).map(|i| i * dot).collect(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

/// The byte offset of every grapheme's start in `text`, then its length.
fn segment(text: &str) -> Vec<usize> {
    UnicodeSegmentation::grapheme_indices(text, true)
        .map(|(start, _)| start)
        .chain(std::iter::once(text.len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Platform, Value};

    #[test]
    fn edits_keep_the_graphemes() {
        let mut value = Value::new("ae\u{301}🇧🇷z");
        assert_eq!(value.len(), 4);

        value.insert_many(2, Value::new("xy"));
        assert_eq!(value.to_string(), "ae\u{301}xy🇧🇷z");
        assert_eq!(value.select(1, 4).to_string(), "e\u{301}xy");

        value.remove_many(1, 3);
        assert_eq!(value.to_string(), "ay🇧🇷z");
        value.remove(2);
        assert_eq!(value, Value::new("ayz"));

        value.insert(1, '\u{301}');
        assert_eq!(value.len(), 3, "the mark joins the a");
        assert_eq!(value.secure().to_string(), "•••");
    }

    #[test]
    fn an_insert_segments_like_the_whole_text() {
        let text = "a🇧🇷🇺🇸e\u{301}👩\u{200d}💻 z";
        for c in ['x', '\u{301}', '🇫', '\u{200d}', ' '] {
            for index in 0..=Value::new(text).len() {
                let mut value = Value::new(text);
                value.insert(index, c);
                assert_eq!(value, Value::new(&value.to_string()), "{c:?} at {index}");
            }
        }
    }

    #[test]
    fn word_motions_count_graphemes() {
        let value = Value::new("héllo  wörld");
        assert_eq!(value.previous_start_of_word(12), 7);
        assert_eq!(value.previous_start_of_word(7), 0);
        assert_eq!(value.next_end_of_word(0), 5);
        assert_eq!(value.next_end_of_word(5), 12);
        assert_eq!(value.next_start_of_word(0), 7);
        assert_eq!(value.word_right(0, Platform::Mac), 5);
        assert_eq!(value.word_left(12, Platform::Linux), 7);
        assert_eq!(value.word_right(0, Platform::Windows), 7);
    }
}
