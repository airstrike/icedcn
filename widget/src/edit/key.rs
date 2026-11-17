//! What a key press does in a focused text widget, on each platform.
//!
//! The bindings follow the platform's own text fields: Cocoa's
//! `StandardKeyBinding.dict` on macOS, RichEdit on Windows, and GTK on
//! Linux, as browsers do in their text fields on each.
use iced_core::keyboard::key::{Named, Physical};
use iced_core::keyboard::{Key, Modifiers};
use iced_core::text::editor::{Binding, KeyPress};

use super::{Command, Platform, kill};

/// What a key press does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Intent {
    /// Run an editing command.
    Command(Command),
    /// Move the caret, dropping the selection.
    Move(Motion),
    /// Move the caret, keeping the selection's anchor.
    Select(Motion),
    /// Delete the selection, or from the caret to where the motion lands.
    Delete(Motion),
    /// Delete to the end of the paragraph, or the line break at its end,
    /// into the kill buffer.
    Kill,
    /// Insert the kill buffer.
    Yank,
    /// Swap the characters around the caret.
    Transpose,
    /// Break the line after the caret.
    OpenLine,
    /// Drop the selection, keeping the caret.
    Deselect,
    /// Scroll without moving the caret.
    Scroll(Scroll),
}

/// Where a motion takes the caret.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Motion {
    Left,
    Right,
    /// The start of the word before the caret.
    WordLeft,
    /// The end of the word after the caret; on Windows, the start of the
    /// next word.
    WordRight,
    /// An edge of the visual line: a wrapped line has one at every wrap.
    LineStart,
    LineEnd,
    /// An edge of the paragraph, between line breaks.
    ParagraphStart,
    ParagraphEnd,
    /// The start of the paragraph, or of the one before at its start.
    ParagraphBackward,
    /// The end of the paragraph, or of the one after at its end; on
    /// Windows, the start of the next paragraph.
    ParagraphForward,
    Up,
    Down,
    PageUp,
    PageDown,
    DocumentStart,
    DocumentEnd,
}

/// A scroll that leaves the caret where it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Scroll {
    Top,
    Bottom,
    PageUp,
    PageDown,
}

/// What `key` does in a focused text widget on `platform`; `lines` for a
/// multi-line one. Typing, Enter, Tab and Escape are left to the widget.
pub(crate) fn resolve(
    key: &Key,
    physical: Physical,
    modifiers: Modifiers,
    platform: Platform,
    lines: bool,
) -> Option<Intent> {
    let intent = match platform {
        Platform::Mac => mac(key, physical, modifiers, lines),
        Platform::Windows | Platform::Linux => pc(key, physical, modifiers, platform, lines),
    };

    if intent != Some(Intent::Kill) {
        kill::close();
    }

    intent
}

fn moving(motion: Motion, shift: bool) -> Intent {
    if shift {
        Intent::Select(motion)
    } else {
        Intent::Move(motion)
    }
}

fn mac(key: &Key, physical: Physical, modifiers: Modifiers, lines: bool) -> Option<Intent> {
    let shift = modifiers.shift();
    let control = modifiers.control();
    let option = modifiers.alt();
    let command = modifiers.logo();

    if let Key::Named(named) = key {
        return Some(match named {
            Named::Backspace if command => Intent::Delete(Motion::LineStart),
            Named::Backspace if option => Intent::Delete(Motion::WordLeft),
            Named::Backspace => Intent::Delete(Motion::Left),
            Named::Delete if command => Intent::Delete(Motion::LineEnd),
            Named::Delete if option => Intent::Delete(Motion::WordRight),
            Named::Delete => Intent::Delete(Motion::Right),
            Named::ArrowLeft if command || control => moving(Motion::LineStart, shift),
            Named::ArrowRight if command || control => moving(Motion::LineEnd, shift),
            Named::ArrowLeft if option => moving(Motion::WordLeft, shift),
            Named::ArrowRight if option => moving(Motion::WordRight, shift),
            Named::ArrowLeft => moving(Motion::Left, shift),
            Named::ArrowRight => moving(Motion::Right, shift),
            Named::ArrowUp if command => moving(Motion::DocumentStart, shift),
            Named::ArrowDown if command => moving(Motion::DocumentEnd, shift),
            Named::ArrowUp if control => Intent::Scroll(Scroll::PageUp),
            Named::ArrowDown if control => Intent::Scroll(Scroll::PageDown),
            Named::ArrowUp if option => moving(Motion::ParagraphBackward, shift),
            Named::ArrowDown if option => moving(Motion::ParagraphForward, shift),
            Named::ArrowUp => moving(Motion::Up, shift),
            Named::ArrowDown => moving(Motion::Down, shift),
            Named::Home if shift => Intent::Select(Motion::DocumentStart),
            Named::End if shift => Intent::Select(Motion::DocumentEnd),
            Named::Home | Named::End if option => return None,
            Named::Home => Intent::Scroll(Scroll::Top),
            Named::End => Intent::Scroll(Scroll::Bottom),
            Named::PageUp if shift => Intent::Select(Motion::PageUp),
            Named::PageDown if shift => Intent::Select(Motion::PageDown),
            Named::PageUp if option => Intent::Move(Motion::PageUp),
            Named::PageDown if option => Intent::Move(Motion::PageDown),
            Named::PageUp => Intent::Scroll(Scroll::PageUp),
            Named::PageDown => Intent::Scroll(Scroll::PageDown),
            _ => return None,
        });
    }

    let letter = key.to_latin(physical)?.to_ascii_lowercase();

    if command && !control {
        return Some(Intent::Command(match letter {
            'x' => Command::Cut,
            'c' => Command::Copy,
            'v' if !option => Command::Paste,
            'a' => Command::SelectAll,
            'z' if shift => Command::Redo,
            'z' => Command::Undo,
            _ => return None,
        }));
    }

    if !control || command {
        return None;
    }

    // The Emacs bindings of Cocoa's text system.
    Some(if option {
        match letter {
            'b' => moving(Motion::WordLeft, shift),
            'f' => moving(Motion::WordRight, shift),
            'h' => Intent::Delete(Motion::WordLeft),
            _ => return None,
        }
    } else {
        match letter {
            'a' => moving(Motion::ParagraphStart, shift),
            'e' => moving(Motion::ParagraphEnd, shift),
            'b' => moving(Motion::Left, shift),
            'f' => moving(Motion::Right, shift),
            'p' => moving(Motion::Up, shift),
            'n' => moving(Motion::Down, shift),
            'v' => moving(Motion::PageDown, shift),
            'h' if !shift => Intent::Delete(Motion::Left),
            'd' if !shift => Intent::Delete(Motion::Right),
            'k' if !shift => Intent::Kill,
            'y' if !shift => Intent::Yank,
            't' if !shift => Intent::Transpose,
            'o' if !shift && lines => Intent::OpenLine,
            _ => return None,
        }
    })
}

fn pc(
    key: &Key,
    physical: Physical,
    modifiers: Modifiers,
    platform: Platform,
    lines: bool,
) -> Option<Intent> {
    if modifiers.logo() {
        return None;
    }

    let shift = modifiers.shift();
    let control = modifiers.control();
    let alt = modifiers.alt();
    let linux = platform == Platform::Linux;
    let windows = platform == Platform::Windows;

    if let Key::Named(named) = key {
        if alt {
            return match named {
                Named::Backspace if windows && !control => Some(Intent::Command(if shift {
                    Command::Redo
                } else {
                    Command::Undo
                })),
                _ => None,
            };
        }

        return Some(match named {
            Named::Insert if control && !shift => Intent::Command(Command::Copy),
            Named::Insert if shift => Intent::Command(Command::Paste),
            Named::Delete if shift && !control => Intent::Command(Command::Cut),
            Named::Delete if control && shift && linux => Intent::Delete(Motion::ParagraphEnd),
            Named::Delete if control => Intent::Delete(Motion::WordRight),
            Named::Delete => Intent::Delete(Motion::Right),
            Named::Backspace if control && shift && linux => Intent::Delete(Motion::ParagraphStart),
            Named::Backspace if control => Intent::Delete(Motion::WordLeft),
            Named::Backspace => Intent::Delete(Motion::Left),
            Named::ArrowLeft if control => moving(Motion::WordLeft, shift),
            Named::ArrowRight if control => moving(Motion::WordRight, shift),
            Named::ArrowLeft => moving(Motion::Left, shift),
            Named::ArrowRight => moving(Motion::Right, shift),
            Named::ArrowUp | Named::ArrowDown | Named::PageUp | Named::PageDown if !lines => {
                return None;
            }
            Named::ArrowUp if control => moving(Motion::ParagraphBackward, shift),
            Named::ArrowDown if control => moving(Motion::ParagraphForward, shift),
            Named::ArrowUp => moving(Motion::Up, shift),
            Named::ArrowDown => moving(Motion::Down, shift),
            Named::Home if control => moving(Motion::DocumentStart, shift),
            Named::End if control => moving(Motion::DocumentEnd, shift),
            Named::Home => moving(Motion::LineStart, shift),
            Named::End => moving(Motion::LineEnd, shift),
            // Browsers switch tabs on Control+PageUp and Control+PageDown.
            Named::PageUp | Named::PageDown if control => return None,
            Named::PageUp => moving(Motion::PageUp, shift),
            Named::PageDown => moving(Motion::PageDown, shift),
            _ => return None,
        });
    }

    // AltGr arrives as Control+Alt on Windows, typing a character.
    if !control || alt {
        return None;
    }

    let letter = key.to_latin(physical)?.to_ascii_lowercase();

    Some(match letter {
        'a' if shift && linux => Intent::Deselect,
        '\\' if linux => Intent::Deselect,
        '/' if linux => Intent::Command(Command::SelectAll),
        'u' if linux && !lines && !shift => Intent::Delete(Motion::ParagraphStart),
        'x' => Intent::Command(Command::Cut),
        'c' => Intent::Command(Command::Copy),
        'v' => Intent::Command(Command::Paste),
        'a' => Intent::Command(Command::SelectAll),
        'z' if shift => Intent::Command(Command::Redo),
        'z' => Intent::Command(Command::Undo),
        'y' => Intent::Command(Command::Redo),
        _ => return None,
    })
}

/// Whether `modifiers` make a key press a shortcut rather than typing on
/// `platform`.
pub(crate) fn is_shortcut(modifiers: Modifiers, platform: Platform) -> bool {
    match platform {
        Platform::Mac => modifiers.logo() || modifiers.control(),
        // AltGr arrives as Control+Alt on Windows, typing a character.
        Platform::Windows | Platform::Linux => {
            modifiers.logo() || (modifiers.control() && !modifiers.alt())
        }
    }
}

/// The editor binding of the keys [`resolve`] leaves to a multi-line
/// widget: typing and Enter. Escape and Tab stay unbound, for the
/// surrounding interface.
pub(crate) fn binding<Message>(press: KeyPress) -> Option<Binding<Message>> {
    if !press.is_focused {
        return None;
    }
    let platform = Platform::current();

    match press.key.as_ref() {
        // Command+Return inserts nothing on macOS; apps take it to send.
        Key::Named(Named::Enter) if platform == Platform::Mac && press.modifiers.logo() => None,
        Key::Named(Named::Enter) => Some(Binding::Enter),
        _ if is_shortcut(press.modifiers, platform) => None,
        _ => {
            let c = press.text?.chars().find(|c| !c.is_control())?;
            Some(Binding::Insert(c))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Intent, Motion, Scroll, resolve};
    use crate::edit::{Command, Platform};
    use iced_core::keyboard::key::{Code, Named, Physical};
    use iced_core::keyboard::{Key, Modifiers};

    fn named(named: Named, modifiers: Modifiers, platform: Platform) -> Option<Intent> {
        resolve(
            &Key::Named(named),
            Physical::Code(Code::F35),
            modifiers,
            platform,
            true,
        )
    }

    fn letter(c: &str, code: Code, modifiers: Modifiers, platform: Platform) -> Option<Intent> {
        resolve(
            &Key::Character(c.into()),
            Physical::Code(code),
            modifiers,
            platform,
            true,
        )
    }

    #[test]
    fn word_motions_take_each_platforms_modifier() {
        let left = Some(Intent::Move(Motion::WordLeft));
        assert_eq!(named(Named::ArrowLeft, Modifiers::ALT, Platform::Mac), left);
        assert_eq!(
            named(Named::ArrowLeft, Modifiers::CTRL, Platform::Windows),
            left
        );
        assert_eq!(
            named(Named::ArrowLeft, Modifiers::CTRL, Platform::Linux),
            left
        );
        assert_eq!(
            named(Named::ArrowLeft, Modifiers::CTRL, Platform::Mac),
            Some(Intent::Move(Motion::LineStart))
        );
    }

    #[test]
    fn control_letters_are_emacs_on_a_mac_and_commands_elsewhere() {
        let ctrl_a = |platform| letter("a", Code::KeyA, Modifiers::CTRL, platform);
        assert_eq!(
            ctrl_a(Platform::Mac),
            Some(Intent::Move(Motion::ParagraphStart))
        );
        assert_eq!(
            ctrl_a(Platform::Windows),
            Some(Intent::Command(Command::SelectAll))
        );
        assert_eq!(
            letter("k", Code::KeyK, Modifiers::CTRL, Platform::Mac),
            Some(Intent::Kill)
        );
        assert_eq!(
            letter("a", Code::KeyA, Modifiers::LOGO, Platform::Mac),
            Some(Intent::Command(Command::SelectAll))
        );
        assert_eq!(
            letter("y", Code::KeyY, Modifiers::LOGO, Platform::Mac),
            None,
            "Command+Y is not redo on a Mac"
        );
    }

    #[test]
    fn home_and_end_scroll_on_a_mac_and_move_elsewhere() {
        assert_eq!(
            named(Named::Home, Modifiers::empty(), Platform::Mac),
            Some(Intent::Scroll(Scroll::Top))
        );
        assert_eq!(
            named(Named::Home, Modifiers::SHIFT, Platform::Mac),
            Some(Intent::Select(Motion::DocumentStart))
        );
        assert_eq!(
            named(Named::Home, Modifiers::empty(), Platform::Linux),
            Some(Intent::Move(Motion::LineStart))
        );
    }

    #[test]
    fn shift_delete_cuts_only_on_a_pc() {
        assert_eq!(
            named(Named::Delete, Modifiers::SHIFT, Platform::Mac),
            Some(Intent::Delete(Motion::Right))
        );
        assert_eq!(
            named(Named::Delete, Modifiers::SHIFT, Platform::Windows),
            Some(Intent::Command(Command::Cut))
        );
        assert_eq!(
            named(
                Named::Backspace,
                Modifiers::ALT | Modifiers::SHIFT,
                Platform::Windows
            ),
            Some(Intent::Command(Command::Redo))
        );
        assert_eq!(
            named(Named::Backspace, Modifiers::ALT, Platform::Mac),
            Some(Intent::Delete(Motion::WordLeft))
        );
    }
}
