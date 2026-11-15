//! Accent and chart color families for a design.

use crate::{Color, oklch};

/// A named color family, usable for controls or as five ordered chart tokens.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Accent {
    #[default]
    Neutral,
    Blue,
    Violet,
    Green,
    Rose,
    Orange,
}

impl Accent {
    /// The control accent in the selected appearance.
    pub fn color(self, dark: bool) -> Color {
        let (chroma, hue) = match self {
            Self::Blue => (0.19, 260.0),
            Self::Violet => (0.20, 295.0),
            Self::Green => (0.15, 150.0),
            Self::Rose => (0.20, 15.0),
            Self::Orange => (0.16, 55.0),
            Self::Neutral => (0.0, 0.0),
        };
        oklch(if dark { 0.76 } else { 0.50 }, chroma, hue)
    }

    /// Chart-1 through chart-5, shared by both appearances.
    pub fn chart(self) -> [Color; 5] {
        match self {
            Self::Neutral => [
                oklch(0.87, 0.0, 0.0),
                oklch(0.556, 0.0, 0.0),
                oklch(0.439, 0.0, 0.0),
                oklch(0.371, 0.0, 0.0),
                oklch(0.269, 0.0, 0.0),
            ],
            Self::Violet => [
                oklch(0.811, 0.111, 293.571),
                oklch(0.606, 0.25, 292.717),
                oklch(0.541, 0.281, 293.009),
                oklch(0.491, 0.27, 292.581),
                oklch(0.432, 0.232, 292.759),
            ],
            Self::Green => [
                oklch(0.871, 0.15, 154.449),
                oklch(0.723, 0.219, 149.579),
                oklch(0.627, 0.194, 149.214),
                oklch(0.527, 0.154, 150.069),
                oklch(0.448, 0.119, 151.328),
            ],
            Self::Rose => [
                oklch(0.81, 0.117, 11.638),
                oklch(0.645, 0.246, 16.439),
                oklch(0.586, 0.253, 17.585),
                oklch(0.514, 0.222, 16.935),
                oklch(0.455, 0.188, 13.697),
            ],
            Self::Orange => [
                oklch(0.837, 0.128, 66.29),
                oklch(0.705, 0.213, 47.604),
                oklch(0.646, 0.222, 41.116),
                oklch(0.553, 0.195, 38.402),
                oklch(0.47, 0.157, 37.304),
            ],
            Self::Blue => [
                oklch(0.809, 0.105, 251.813),
                oklch(0.623, 0.214, 259.815),
                oklch(0.546, 0.245, 262.881),
                oklch(0.488, 0.243, 264.376),
                oklch(0.424, 0.199, 265.638),
            ],
        }
    }
}

impl From<Accent> for [Color; 5] {
    fn from(accent: Accent) -> Self {
        accent.chart()
    }
}
