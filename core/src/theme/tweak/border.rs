//! Border tweaks. The radius is [`crate::radius`].

use super::{Token, Tweak};

/// Set the border color.
pub fn color(token: impl Into<Token>) -> Tweak {
    Tweak::BorderColor(token.into())
}

/// Set the border width in pixels.
pub fn width(width: f32) -> Tweak {
    Tweak::BorderWidth(width)
}
