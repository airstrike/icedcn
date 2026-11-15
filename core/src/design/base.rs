//! Surface color families for a design.

use super::Design;
use crate::Color;

/// A neutral surface palette; lightness and transparency follow the design mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Base {
    #[default]
    Neutral,
    Slate,
    Stone,
    Zinc,
}

impl Base {
    pub(super) fn apply(self, design: &mut Design) {
        let (chroma, hue) = match self {
            Self::Slate => (0.016, 260.0),
            Self::Stone => (0.008, 75.0),
            Self::Zinc => (0.005, 285.0),
            Self::Neutral => (0.0, 0.0),
        };
        let tint = |color: &mut Color| {
            if color.l > 0.02 && color.l < 0.99 && color.a == 1.0 {
                color.c = chroma;
                color.h = hue;
            }
        };
        for palette in [
            &mut design.neutral,
            &mut design.secondary,
            &mut design.muted,
            &mut design.accent,
            &mut design.card,
            &mut design.popover,
            &mut design.code,
        ] {
            for color in [
                &mut palette.base,
                &mut palette.foreground,
                &mut palette.border,
                &mut palette.ring,
                &mut palette.tint_text,
            ] {
                tint(color);
            }
        }
        for color in [
            &mut design.input,
            &mut design.sidebar.background,
            &mut design.sidebar.foreground,
            &mut design.sidebar.border,
            &mut design.sidebar.ring,
            &mut design.sidebar.accent.base,
            &mut design.sidebar.accent.foreground,
        ] {
            tint(color);
        }
    }
}
