//! Fill and focus-ring tweaks.

use super::{Token, Tweak};

/// Multiply the background's alpha by `factor`; on text, the text color's.
/// No-op when the variant has no background.
pub fn alpha(factor: f32) -> Tweak {
    Tweak::Alpha(factor)
}

/// Replace a translucent resting background with the page background, so
/// the surface below does not show through. Hover and pressed keep the
/// variant's own transitions.
pub fn opaque() -> Tweak {
    Tweak::Opaque
}

/// A focus ring of `width` pixels drawn `offset` pixels outside the edge.
/// Only buttons draw the offset.
pub fn ring(color: impl Into<Token>, width: f32, offset: f32) -> Tweak {
    Tweak::Ring {
        color: color.into(),
        width,
        offset,
    }
}
