//! Avatar geometry and fallback colors. Small, default, and large avatars measure 24, 32, and
//! 40 pixels across. The widget paints the inset edge and outer group ring separately.

use super::Theme;
use super::container::Style;
use crate::{Border, Color};

/// Pill-shape radius — clamps the avatar to a perfect circle regardless of
/// pixel diameter. Same trick `theme::badge` uses.
const PILL_RADIUS: f32 = u32::MAX as f32;

/// Avatar size variant. Geometry only — visual styling is shared across
/// variants and lives in [`fallback_style`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

/// Small (24px) avatar variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sm;

/// Large (40px) avatar variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lg;

/// Geometry for an avatar at a given size.
///
/// All values in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geometry {
    /// Diameter of the circular avatar surface (`size-6` / `size-8` / `size-10`).
    pub diameter: f32,
    /// Font size for fallback initials, in logical pixels.
    pub font_size: f32,
    /// Diameter of the corner badge, in logical pixels.
    pub badge_diameter: f32,
    /// Icon size inside the corner badge. A zero size hides the icon on small avatars.
    pub badge_icon_size: f32,
}

impl Geometry {
    /// Default-size geometry. Public for downstream callers that need to
    /// reach the same constants the widget uses.
    pub const DEFAULT: Self = Self {
        diameter: 32.0,
        font_size: 14.0,
        badge_diameter: 10.0,
        badge_icon_size: 8.0,
    };

    /// Small-size geometry.
    pub const SM: Self = Self {
        diameter: 24.0,
        font_size: 12.0,
        badge_diameter: 8.0,
        badge_icon_size: 0.0,
    };

    /// Large-size geometry.
    pub const LG: Self = Self {
        diameter: 40.0,
        font_size: 14.0,
        badge_diameter: 12.0,
        badge_icon_size: 8.0,
    };
}

impl From<Default> for Geometry {
    fn from(_: Default) -> Self {
        Self::DEFAULT
    }
}

impl From<Sm> for Geometry {
    fn from(_: Sm) -> Self {
        Self::SM
    }
}

impl From<Lg> for Geometry {
    fn from(_: Lg) -> Self {
        Self::LG
    }
}

/// Container style for the circular fallback surface. Set `ring` to draw a two-pixel inset
/// border. Grouped avatars paint their ring outside the bounds.
pub fn fallback_style(theme: &Theme, ring: bool) -> Style {
    let p = theme;
    Style {
        background: Some(p.muted.base.into()),
        text_color: Some(p.muted.foreground),
        border: Border {
            radius: PILL_RADIUS.into(),
            // Legacy container approximation. The Avatar widget paints an
            // actual outer ring separately without reducing the image area.
            width: if ring { 2.0 } else { 0.0 },
            color: if ring {
                p.neutral.base
            } else {
                Color::TRANSPARENT
            },
        },
        ..Style::default()
    }
}

/// Corner badge style with primary fill and a two-pixel ring in the background color.
pub fn badge_style(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.primary.base.into()),
        text_color: Some(p.primary.foreground),
        border: Border {
            radius: PILL_RADIUS.into(),
            width: 2.0,
            color: p.neutral.base,
        },
        ..Style::default()
    }
}

/// Style for the AvatarGroupCount circle — same surface as a fallback
/// avatar plus the 2px page-bg ring (group context is implied by the
/// existence of a count).
pub fn group_count_style(theme: &Theme) -> Style {
    fallback_style(theme, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pill-radius clamp resolves to the same large value across all
    /// variants — geometry is the only difference between them.
    #[test]
    fn fallback_style_radius_is_pill() {
        let theme = Theme::dark();
        let s = fallback_style(&theme, false);
        assert!(s.background.is_some());
        // The radius is a `Radius` of 4 corners; the absolute value is
        // PILL_RADIUS but we only need to confirm it's the pill clamp,
        // not a fractional/rounded-rectangle radius.
        let r: f32 = s.border.radius.top_left;
        assert!(r >= PILL_RADIUS, "expected pill-clamped radius, got {r}",);
    }

    /// In-group avatars carry a 2px ring; stand-alone avatars do not.
    #[test]
    fn ring_toggles_border_width() {
        let theme = Theme::dark();
        assert_eq!(fallback_style(&theme, false).border.width, 0.0);
        assert_eq!(fallback_style(&theme, true).border.width, 2.0);
    }
}
