//! Native Hyozu chart styling from the active icedcn design.

use hyozu::{Color, Pair, palette, text};
use iced_core::{Font, border, color};

use super::Theme;

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
        [
            self.primary.base,
            self.info.base,
            self.success.base,
            self.warning.base,
            self.destructive.base,
        ]
        .into_iter()
        .map(fixed)
        .collect()
    }

    fn seed(&self) -> palette::Seed {
        palette::Seed {
            primary: self.primary.base.into(),
            secondary: self.info.base.into(),
            success: self.success.base.into(),
            warning: self.warning.base.into(),
            danger: self.destructive.base.into(),
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
