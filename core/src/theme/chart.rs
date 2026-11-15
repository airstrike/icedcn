//! Native Hyozu chart styling from the active icedcn design.

use hyozu::{Color, Pair, palette, text};
use iced_core::{Font, Vector, border, color};

use super::{Theme, container};

/// Legacy sRGB approximations of Nova's default chart tokens.
/// Read `theme.chart` for the active design's chart-1 through chart-5 colors.
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
        self.chart.into_iter().map(fixed).collect()
    }

    fn seed(&self) -> palette::Seed {
        palette::Seed {
            primary: self.chart[0].into(),
            secondary: self.chart[1].into(),
            success: self.chart[2].into(),
            warning: self.chart[3].into(),
            danger: self.chart[4].into(),
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

#[cfg(test)]
mod tests {
    use super::tooltip;
    use crate::{Design, FontFamily, Theme, oklch};
    use hyozu::{Color, Design as _, bar, palette};
    use iced_core::{Font, color};

    #[test]
    fn theme_tokens_reach_chart_chrome_and_typography_in_both_modes() {
        for base in [Design::light(), Design::dark()] {
            let mut design = base.with_font_sans(FontFamily::from_name("Chart Test Font"));
            design.muted.foreground = oklch(0.6, 0.1, 150.0);
            design.neutral.border = oklch(0.5, 0.1, 260.0).alpha(0.25);
            let theme = Theme::new(design);
            let resolve = |color: Color| {
                color.resolve(
                    theme.background_color(),
                    theme.text_pair(),
                    &theme.seed(),
                    None,
                )
            };
            assert_eq!(theme.background_color(), theme.neutral.base.into());
            assert_eq!(resolve(theme.text_color()), theme.neutral.foreground.into());
            assert_eq!(resolve(theme.axis_color()), theme.muted.foreground.into());
            assert_eq!(
                resolve(theme.axis_label_color()),
                theme.muted.foreground.into()
            );
            assert_eq!(resolve(theme.grid_color()), theme.neutral.border.into());
            assert_eq!(resolve(theme.divider_color()), theme.neutral.border.into());
            assert_eq!(theme.font(), Font::from("Chart Test Font"));
            for style in [
                theme.axis_text(),
                theme.legend_text(),
                theme.data_label_text(),
            ] {
                assert_eq!(style.resolved_font(Font::DEFAULT), theme.font());
            }
            let annotation = tooltip(&theme);
            assert_eq!(annotation.background, Some(theme.neutral.base.into()));
            assert_eq!(annotation.text_color, Some(theme.neutral.foreground));
        }
    }

    #[test]
    fn cached_marks_resolve_custom_chart_tokens_and_keep_explicit_overrides() {
        let plain = hyozu::bar([1.0]);
        let semantic = hyozu::bar([1.0]).with_color(Color::Secondary);
        let explicit = hyozu::bar([1.0]).with_color(color!(0xff0077));
        for base in [Design::light(), Design::dark()] {
            for hue in [30.0, 150.0, 290.0] {
                let mut design = base.clone();
                design.chart = std::array::from_fn(|i| oklch(0.7 - i as f32 * 0.08, 0.1, hue));
                let theme = Theme::new(design);
                let seed = theme.seed();
                let palette = palette::Resolved::resolve(&palette::Palette::TONAL, &seed, 1);
                let resolve = |series: &bar::Series| {
                    series
                        .resolved_color_at(0, &seed, None, palette.get(0))
                        .resolve(theme.background_color(), theme.text_pair(), &seed, None)
                };
                assert_eq!(resolve(&plain), theme.chart[0].into());
                assert_eq!(resolve(&semantic), theme.chart[1].into());
                assert_eq!(resolve(&explicit), color!(0xff0077));
            }
        }
    }
}
