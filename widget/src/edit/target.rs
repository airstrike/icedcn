//! Key intents carried out in a multi-line editor: the Textarea and the
//! editable code block.
use std::sync::Arc;

use iced_core::text;
use iced_core::text::editor::{self, Action, Cursor, Edit};
use iced_widget::Renderer;
use iced_widget::text_editor::Content;
use unicode_segmentation::UnicodeSegmentation;

use super::key::{Intent, Motion, Scroll};
use super::{Platform, kill, next_word_start};

/// A multi-line editor the intents of key presses run in.
pub(crate) trait Target {
    /// The text and caret being edited.
    fn content(&self) -> &Content<Renderer>;
    /// Whether edits change the text.
    fn editable(&self) -> bool;
    /// Perform an action of the editor, publishing any edit.
    fn perform(&mut self, action: Action);
    /// Put the caret and the selection at `cursor`.
    fn move_to(&mut self, cursor: Cursor);
    /// How many lines a page scrolls.
    fn page(&self) -> i32;
}

/// Carry out `intent` in `target` on `platform`. Commands are left to the
/// widget. Returns whether the key press is used up: a scroll lets an
/// outer scroller see it too.
pub(crate) fn apply(target: &mut impl Target, intent: Intent, platform: Platform) -> bool {
    match intent {
        Intent::Command(_) => return false,
        Intent::Move(motion) => travel(target, motion, false, platform),
        Intent::Select(motion) => travel(target, motion, true, platform),
        Intent::Delete(motion) if target.editable() => delete(target, motion, platform),
        Intent::Kill if target.editable() => cut_to_end(target),
        Intent::Yank if target.editable() => {
            if let Some(text) = kill::yank() {
                target.perform(Action::Edit(Edit::Paste(Arc::new(text))));
            }
        }
        Intent::Transpose if target.editable() => transpose(target),
        Intent::OpenLine if target.editable() => {
            target.perform(Action::Edit(Edit::Enter));
            target.perform(Action::Move(editor::Motion::Left));
        }
        Intent::Delete(_) | Intent::Kill | Intent::Yank | Intent::Transpose | Intent::OpenLine => {}
        Intent::Deselect => {
            let position = target.content().cursor().position;
            target.move_to(Cursor {
                position,
                selection: None,
            });
        }
        Intent::Scroll(scroll) => {
            // Far enough for any text: the editor stops at its ends.
            const ENDS: i32 = 1 << 20;
            let lines = match scroll {
                Scroll::Top => -ENDS,
                Scroll::Bottom => ENDS,
                Scroll::PageUp => -target.page(),
                Scroll::PageDown => target.page(),
            };
            target.perform(Action::Scroll { lines });
            return false;
        }
    }

    true
}

fn key(position: text::Position) -> (usize, usize) {
    (position.line, position.index)
}

fn length(target: &impl Target, line: usize) -> usize {
    target
        .content()
        .line(line)
        .map_or(0, |line| line.text.len())
}

/// Put the caret at `position`, keeping the anchor of the selection when
/// `select`.
fn place(target: &mut impl Target, position: text::Position, select: bool) {
    let cursor = target.content().cursor();
    let anchor = cursor.selection.unwrap_or(cursor.position);

    target.move_to(Cursor {
        position,
        selection: (select && anchor != position).then_some(anchor),
    });
}

fn travel(target: &mut impl Target, motion: Motion, select: bool, platform: Platform) {
    let action = |motion| {
        if select {
            Action::Select(motion)
        } else {
            Action::Move(motion)
        }
    };
    let cursor = target.content().cursor();
    let caret = cursor.position;

    match motion {
        Motion::Left => target.perform(action(editor::Motion::Left)),
        Motion::Right => target.perform(action(editor::Motion::Right)),
        Motion::WordLeft => target.perform(action(editor::Motion::WordLeft)),
        Motion::WordRight if platform == Platform::Windows => {
            let from = match cursor.selection {
                Some(anchor) if !select && key(anchor) > key(caret) => anchor,
                _ => caret,
            };
            let line = length(target, from.line);
            if from.index >= line {
                target.perform(action(editor::Motion::WordRight));
                return;
            }
            let text = target
                .content()
                .line(from.line)
                .map(|line| line.text.to_string())
                .unwrap_or_default();
            let position = text::Position {
                index: next_word_start(&text, from.index),
                ..from
            };
            place(target, position, select);
        }
        Motion::WordRight => target.perform(action(editor::Motion::WordRight)),
        Motion::LineStart => target.perform(action(editor::Motion::Home)),
        Motion::LineEnd => target.perform(action(editor::Motion::End)),
        Motion::PageUp => target.perform(action(editor::Motion::PageUp)),
        Motion::PageDown => target.perform(action(editor::Motion::PageDown)),
        Motion::DocumentStart => target.perform(action(editor::Motion::DocumentStart)),
        Motion::DocumentEnd => target.perform(action(editor::Motion::DocumentEnd)),
        Motion::Up | Motion::Down => {
            let up = motion == Motion::Up;
            // Where the caret leaves from: a selection collapses to an edge.
            let edge = match cursor.selection {
                Some(anchor) if !select && (key(anchor) < key(caret)) == up => anchor,
                _ => caret,
            };
            target.perform(action(if up {
                editor::Motion::Up
            } else {
                editor::Motion::Down
            }));

            // Past the first or the last line, macOS and GTK go to the ends.
            if platform != Platform::Windows && target.content().cursor().position == edge {
                let last = target.content().line_count().saturating_sub(1);
                let end = if up {
                    text::Position { line: 0, index: 0 }
                } else {
                    text::Position {
                        line: last,
                        index: length(target, last),
                    }
                };
                let anchor = cursor.selection.unwrap_or(caret);
                target.move_to(Cursor {
                    position: end,
                    selection: (select && anchor != end).then_some(anchor),
                });
            }
        }
        Motion::ParagraphStart
        | Motion::ParagraphEnd
        | Motion::ParagraphBackward
        | Motion::ParagraphForward => {
            let position = paragraph(target, caret, motion, platform);
            place(target, position, select);
        }
    }
}

/// Where a paragraph motion from `caret` lands.
fn paragraph(
    target: &impl Target,
    caret: text::Position,
    motion: Motion,
    platform: Platform,
) -> text::Position {
    let at = |line, index| text::Position { line, index };
    let end = |line| at(line, length(target, line));
    let lines = target.content().line_count();
    let line = caret.line;

    match motion {
        Motion::ParagraphStart => at(line, 0),
        Motion::ParagraphEnd => end(line),
        Motion::ParagraphBackward if caret.index == 0 && line > 0 => at(line - 1, 0),
        Motion::ParagraphBackward => at(line, 0),
        Motion::ParagraphForward if platform == Platform::Windows => {
            if line + 1 < lines {
                at(line + 1, 0)
            } else {
                end(line)
            }
        }
        Motion::ParagraphForward if caret.index >= length(target, line) && line + 1 < lines => {
            end(line + 1)
        }
        _ => end(line),
    }
}

/// Select from `caret` to `position` and delete it, so undo selects the
/// text again.
fn delete_to(target: &mut impl Target, caret: text::Position, position: text::Position) {
    target.move_to(Cursor {
        position,
        selection: Some(caret),
    });
    target.perform(Action::Edit(Edit::Backspace));
}

fn delete(target: &mut impl Target, motion: Motion, platform: Platform) {
    let cursor = target.content().cursor();
    let caret = cursor.position;
    if cursor.selection.is_some() {
        target.perform(Action::Edit(Edit::Backspace));
        return;
    }

    let backward = matches!(
        motion,
        Motion::Left | Motion::WordLeft | Motion::LineStart | Motion::ParagraphStart
    );
    let one = if backward {
        Edit::Backspace
    } else {
        Edit::Delete
    };

    let position = match motion {
        Motion::Left | Motion::Right => {
            target.perform(Action::Edit(one));
            return;
        }
        Motion::WordLeft => {
            target.perform(Action::Edit(Edit::BackspaceWord));
            return;
        }
        Motion::WordRight if platform != Platform::Windows => {
            target.perform(Action::Edit(Edit::DeleteWord));
            return;
        }
        Motion::WordRight => {
            let text = target
                .content()
                .line(caret.line)
                .map(|line| line.text.to_string())
                .unwrap_or_default();
            if caret.index >= text.len() {
                target.perform(Action::Edit(Edit::Delete));
                return;
            }
            text::Position {
                index: next_word_start(&text, caret.index),
                ..caret
            }
        }
        Motion::LineStart | Motion::LineEnd => {
            target.perform(Action::Move(if backward {
                editor::Motion::Home
            } else {
                editor::Motion::End
            }));
            target.content().cursor().position
        }
        _ => paragraph(target, caret, motion, platform),
    };

    if position == caret {
        target.move_to(cursor);
        target.perform(Action::Edit(one));
    } else {
        delete_to(target, caret, position);
    }
}

/// Kill to the end of the paragraph, or the line break at its end.
fn cut_to_end(target: &mut impl Target) {
    let cursor = target.content().cursor();
    let caret = cursor.position;

    if cursor.selection.is_none() {
        let position = if caret.index < length(target, caret.line) {
            text::Position {
                index: length(target, caret.line),
                ..caret
            }
        } else if caret.line + 1 < target.content().line_count() {
            text::Position {
                line: caret.line + 1,
                index: 0,
            }
        } else {
            return;
        };
        target.move_to(Cursor {
            position,
            selection: Some(caret),
        });
    }

    let Some(text) = target.content().selection() else {
        return;
    };
    target.perform(Action::Edit(Edit::Backspace));
    kill::push(&text);
}

/// Swap the characters around the caret, or the two before it at the end
/// of a line, and move past them.
fn transpose(target: &mut impl Target) {
    let cursor = target.content().cursor();
    let caret = cursor.position;
    if cursor.selection.is_some() || caret.index == 0 {
        return;
    }
    let text = target
        .content()
        .line(caret.line)
        .map(|line| line.text.to_string())
        .unwrap_or_default();

    let Some((start, middle, end)) = transposed(&text, caret.index) else {
        return;
    };
    let swapped = format!("{}{}", &text[middle..end], &text[start..middle]);

    target.move_to(Cursor {
        position: text::Position {
            index: end,
            ..caret
        },
        selection: Some(text::Position {
            index: start,
            ..caret
        }),
    });
    target.perform(Action::Edit(Edit::Paste(Arc::new(swapped))));
}

/// The byte bounds of the two graphemes a transpose at `index` of `text`
/// swaps: the one before and the one after, or the two before at the end.
pub(crate) fn transposed(text: &str, index: usize) -> Option<(usize, usize, usize)> {
    let bounds: Vec<usize> = text
        .grapheme_indices(true)
        .map(|(start, _)| start)
        .chain(std::iter::once(text.len()))
        .collect();
    let at = bounds.iter().position(|&bound| bound == index)?;
    let at = if index == text.len() {
        at.checked_sub(1)?
    } else {
        at
    };

    Some((
        *bounds.get(at.checked_sub(1)?)?,
        bounds[at],
        *bounds.get(at + 1)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::transposed;

    #[test]
    fn a_transpose_swaps_around_the_caret_or_before_the_end() {
        let swap = |text: &str, index: usize| {
            transposed(text, index).map(|(start, middle, end)| {
                format!(
                    "{}{}{}{}",
                    &text[..start],
                    &text[middle..end],
                    &text[start..middle],
                    &text[end..]
                )
            })
        };
        assert_eq!(swap("abcd", 2).as_deref(), Some("acbd"));
        assert_eq!(swap("abcd", 4).as_deref(), Some("abdc"));
        assert_eq!(swap("abcd", 0), None);
        assert_eq!(swap("a", 1), None);
        assert_eq!(swap("ae\u{301}x", 1).as_deref(), Some("e\u{301}ax"));
    }
}
