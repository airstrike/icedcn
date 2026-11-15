//! Native Hyozu chart styling from the active icedcn design.

use hyozu::{Color, Pair, palette, text};
use iced_core::{Font, Vector, border, color};

use super::{Theme, container};

/// Nova's chart tokens, in chart-1 through chart-5 order.
/// These remain blue in both light and dark mode, independently of control colors.
pub const COLORS: [iced_core::Color; 5] = [
    color!(0x8ec5ff),
    color!(0x2b7fff),
    color!(0x155dfb),
    color!(0x1447e6),
    color!(0x193cb8),
];

/// The compact chart annotation surface: background, half-opacity border and shadow-xl.
pub fn tooltip(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(theme.neutral.base.into()),
        text_color: Some(theme.neutral.foreground),
        border: crate::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: theme.neutral.border.scale_alpha(0.5),
        },
        shadow: crate::Shadow {
            color: color!(0x000000, 0.1).into(),
            offset: Vector::new(0.0, 20.0),
            blur_radius: 25.0,
        },
        ..container::Style::default()
    }
}

impl hyozu::Design for Theme {
    fn background_color(&self) -> iced_core::Color {
        self.neutral.base.into()
    }

    fn text_color(&self) -> Color {
        fixed(self.neutral.foreground)
    }

    fn text_pair(&self) -> Pair {
        Pair::new(color!(0x000000), color!(0xffffff))
    }

    fn data_colors(&self) -> Vec<Color> {
        COLORS.into_iter().map(Into::into).collect()
    }

    fn seed(&self) -> palette::Seed {
        palette::Seed {
            primary: COLORS[0],
            secondary: COLORS[1],
            success: COLORS[2],
            warning: COLORS[3],
            danger: COLORS[4],
            background: self.neutral.base.into(),
        }
    }

    fn divider_color(&self) -> Color {
        fixed(self.neutral.border)
    }

    fn axis_color(&self) -> Color {
        fixed(self.muted.foreground)
    }

    fn axis_label_color(&self) -> Color {
        fixed(self.muted.foreground)
    }

    fn grid_color(&self) -> Color {
        fixed(self.neutral.border)
    }

    fn minor_grid_color(&self) -> Color {
        fixed(self.neutral.border.alpha(0.5))
    }

    fn font(&self) -> Font {
        self.font_sans.regular
    }

    fn font_size(&self) -> f32 {
        12.0
    }

    fn legend_text(&self) -> text::Style {
        text::Style::new().font(self.font_sans.regular).size(12.0)
    }

    fn corners(&self) -> border::Radius {
        4.0.into()
    }
}

fn fixed(value: crate::Color) -> Color {
    iced_core::Color::from(value).into()
}
