//! Key intents carried out in a multi-line editor: the Textarea and the
//! editable code block.
use std::sync::Arc;

use iced_core::alignment::Vertical;
use iced_core::text;
use iced_core::text::editor::{self, Action, Binding, Cursor, Edit, KeyPress};
use iced_core::{Font, Point, Rectangle, Size};
use iced_widget::Renderer;
use iced_widget::text_editor::Content;
use unicode_segmentation::UnicodeSegmentation;

use super::key::{Intent, Motion, Scroll};
use super::{Platform, kill, next_word_start, word_left};

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
    /// How many lines show at once.
    fn page(&self) -> i32;
    /// Scroll the view: the editor's own, or the scrollables around it.
    fn reveal(&mut self, reveal: Reveal);
    /// Switch between inserting and typing over the text.
    fn overwrite(&mut self);
}

/// What a key press asks the view to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Reveal {
    /// The caret, scrolling the least.
    Caret,
    /// The caret, in the middle.
    Center,
    /// A scroll that leaves the caret where it is.
    Scroll(Scroll),
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
            target.reveal(Reveal::Scroll(scroll));
            return true;
        }
        Intent::Center => {
            target.reveal(Reveal::Center);
            return true;
        }
        Intent::Overwrite => {
            target.overwrite();
            return true;
        }
    }

    // A page key scrolls the view with the caret, by a page.
    target.reveal(match intent {
        Intent::Move(Motion::PageUp) | Intent::Select(Motion::PageUp) => {
            Reveal::Scroll(Scroll::PageUp)
        }
        Intent::Move(Motion::PageDown) | Intent::Select(Motion::PageDown) => {
            Reveal::Scroll(Scroll::PageDown)
        }
        _ => Reveal::Caret,
    });
    true
}

fn key(position: text::Position) -> (usize, usize) {
    (position.line, position.index)
}

fn line_text(target: &impl Target, line: usize) -> String {
    target
        .content()
        .line(line)
        .map(|line| line.text.to_string())
        .unwrap_or_default()
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
        Motion::WordLeft if platform == Platform::Windows => {
            let from = match cursor.selection {
                Some(anchor) if !select && key(anchor) < key(caret) => anchor,
                _ => caret,
            };
            if from.index == 0 {
                target.perform(action(editor::Motion::WordLeft));
                return;
            }
            let text = line_text(target, from.line);
            let position = text::Position {
                index: word_left(&text, from.index, platform),
                ..from
            };
            place(target, position, select);
        }
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
        // A page is what shows of the editor, which may be taller than its
        // scrollable around it.
        Motion::PageUp | Motion::PageDown => {
            let motion = if motion == Motion::PageUp {
                editor::Motion::Up
            } else {
                editor::Motion::Down
            };
            for _ in 0..target.page().saturating_sub(1).max(1) {
                target.perform(action(motion));
            }
        }
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
        Motion::WordLeft if platform == Platform::Windows => {
            if caret.index == 0 {
                target.perform(Action::Edit(Edit::Backspace));
                return;
            }
            let text = line_text(target, caret.line);
            text::Position {
                index: word_left(&text, caret.index, platform),
                ..caret
            }
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

/// The editor binding of typing and Enter, typing over the character after
/// the caret when `over` holds the content, as overwrite mode does.
pub(crate) fn typing<Message>(
    press: KeyPress,
    over: Option<&Content<Renderer>>,
) -> Option<Binding<Message>> {
    let binding = super::binding(press)?;

    match (binding, over) {
        (Binding::Insert(c), Some(content)) => {
            let cursor = content.cursor();
            let at_end = content
                .line(cursor.position.line)
                .is_none_or(|line| cursor.position.index >= line.text.len());

            Some(if cursor.selection.is_some() || at_end {
                Binding::Insert(c)
            } else {
                Binding::Sequence(vec![
                    Binding::Select(editor::Motion::Right),
                    Binding::Insert(c),
                ])
            })
        }
        (binding, _) => Some(binding),
    }
}

/// The width of the block caret over the character after the caret in
/// `content`, or over a space at the end of a line.
pub(crate) fn block_width(
    content: &Content<Renderer>,
    font: Font,
    size: f32,
    renderer: &Renderer,
) -> f32 {
    let position = content.cursor().position;
    let next = content
        .line(position.line)
        .and_then(|line| {
            line.text[position.index.min(line.text.len())..]
                .graphemes(true)
                .next()
                .map(str::to_owned)
        })
        .unwrap_or_else(|| " ".to_owned());

    // A trailing space adds no width to a measured line, so measure up to
    // a sentinel and take the sentinel away.
    let width = |content: &str| {
        crate::text::min_bounds::<Renderer>(text::Text {
            content,
            bounds: Size::INFINITE,
            size: size.into(),
            line_height: text::LineHeight::default(),
            font,
            align_x: text::Alignment::Default,
            align_y: Vertical::Top,
            shaping: text::Shaping::Advanced,
            wrapping: text::Wrapping::None,
            ellipsis: text::Ellipsis::None,
            letter_spacing: Default::default(),
            font_features: Vec::new(),
            font_variations: Vec::new(),
            weight: None,
            hint_factor: iced_core::Renderer::hint_factor(renderer),
        })
        .width
    };

    (width(&format!("{next}x")) - width("x")).max(1.0)
}

/// The area the scrollables around an editor laid out at `bounds` must show
/// for `reveal`, with its caret at `caret` and `viewport` visible.
pub(crate) fn area(
    reveal: Reveal,
    bounds: Rectangle,
    viewport: Rectangle,
    caret: Rectangle,
) -> Option<Rectangle> {
    // Near an edge of the editor, show the edge too: its padding and the
    // frame around it.
    let margin = caret.height * 3.0;
    let edge = |y: f32| {
        Rectangle::new(
            Point::new(bounds.x, y - margin),
            Size::new(1.0, margin * 2.0),
        )
    };
    let page = |sign: f32| Rectangle {
        y: viewport.y + sign * viewport.height,
        ..viewport
    };

    match reveal {
        Reveal::Caret => {
            let top = if caret.y - bounds.y < margin {
                bounds.y - margin
            } else {
                caret.y
            };
            let bottom = if bounds.y + bounds.height - (caret.y + caret.height) < margin {
                bounds.y + bounds.height + margin
            } else {
                caret.y + caret.height
            };
            Some(Rectangle {
                y: top,
                height: bottom - top,
                ..caret
            })
        }
        Reveal::Center => Some(Rectangle {
            y: caret.center_y() - viewport.height / 2.0,
            height: viewport.height,
            ..caret
        }),
        Reveal::Scroll(Scroll::Top) => Some(edge(bounds.y)),
        Reveal::Scroll(Scroll::Bottom) => Some(edge(bounds.y + bounds.height)),
        Reveal::Scroll(Scroll::PageUp) => page(-1.0).intersection(&bounds),
        Reveal::Scroll(Scroll::PageDown) => page(1.0).intersection(&bounds),
    }
}

/// The lines a scroll of the editor's own text moves, or `None` for the
/// caret, which the editor keeps in view itself.
pub(crate) fn lines(reveal: Reveal, page: i32, caret: f32, middle: f32, line: f32) -> Option<i32> {
    // Far enough for any text: the editor stops at its ends.
    const ENDS: i32 = 1 << 20;

    match reveal {
        Reveal::Caret => None,
        Reveal::Center => Some(((caret - middle) / line).round() as i32),
        Reveal::Scroll(Scroll::Top) => Some(-ENDS),
        Reveal::Scroll(Scroll::Bottom) => Some(ENDS),
        Reveal::Scroll(Scroll::PageUp) => Some(-page),
        Reveal::Scroll(Scroll::PageDown) => Some(page),
    }
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
