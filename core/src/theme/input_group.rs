//! InputGroup styling - nova style.
//!
//! The group paints the Input's surface (border, focus ring, invalid and
//! disabled states, from [`text_input`]) around its addons; the control
//! inside is bare: `rounded-none border-0 bg-transparent ring-0`.

use super::Theme;
use super::text_input::{self, Status, Style};
use crate::{Background, Border, Color};

/// An Input inside a group: the Input's text colors on no surface.
pub fn control(theme: &Theme, status: Status) -> Style {
    let mut ink = text_input::default(theme, status);
    if matches!(status, Status::Disabled) {
        // The control and its disabled group each apply 50% opacity.
        ink.value = ink.value.scale_alpha(0.5);
        ink.placeholder = ink.placeholder.scale_alpha(0.5);
    }
    Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        ring: None,
        ..ink
    }
}

/// A Textarea inside a group: the textarea's text colors on no surface.
pub fn textarea(
    theme: &Theme,
    status: iced_widget::text_editor::Status,
) -> iced_widget::text_editor::Style {
    iced_widget::text_editor::Style {
        background: iced_core::Background::Color(iced_core::Color::TRANSPARENT),
        border: iced_core::Border::default(),
        ..super::textarea::default(theme, status)
    }
}
