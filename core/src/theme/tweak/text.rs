//! Typography tweaks. Widgets without text drop them.

use iced_core::alignment::Horizontal;
use iced_core::font::Weight;
use iced_core::{Font, Pixels};

use super::Tweak;

/// Set the font size.
pub fn size(size: impl Into<Pixels>) -> Tweak {
    Tweak::Size(size.into())
}

/// Set the font weight.
pub fn weight(weight: impl Into<Weight>) -> Tweak {
    Tweak::Weight(weight.into())
}

/// Set the font family.
pub fn family(font: impl Into<Font>) -> Tweak {
    Tweak::Family(font.into())
}

/// Set the line height, relative to the font size.
pub fn line_height(height: f32) -> Tweak {
    Tweak::LineHeight(height)
}

/// Set the italic style.
pub fn italic() -> Tweak {
    Tweak::Italic
}

/// Set the horizontal text alignment.
pub fn align(align: impl Into<Horizontal>) -> Tweak {
    Tweak::Align(align.into())
}
