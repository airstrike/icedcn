//! Theme choices shared by the preview and the exported Rust configuration.

use iced::Font;
use iced::font::{Family, Weight};
use icedcn::Theme;
use icedcn::core::{Color, Design, oklch};

use crate::Mode;

/// Supported surface palettes.
pub const BASES: &[&str] = &["Neutral", "Slate", "Stone", "Zinc"];
/// Supported primary and chart accents.
pub const ACCENTS: &[&str] = &["Neutral", "Blue", "Violet", "Green", "Rose", "Orange"];
/// Sidebar selection treatments.
pub const MENUS: &[&str] = &["Subtle", "Bold"];

/// Customizations that can be represented by icedcn's public design API.
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub base: &'static str,
    pub accent: &'static str,
    pub chart: &'static str,
    pub menu: &'static str,
    pub font: Font,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            base: "Neutral",
            accent: "Neutral",
            chart: "Neutral",
            menu: "Subtle",
            font: Font::DEFAULT,
        }
    }
}

impl Settings {
    /// Build the live design without mutating the host application's font defaults.
    pub fn theme(&self, mode: Mode) -> Theme {
        let dark = mode == Mode::Dark;
        let mut design = if dark {
            Design::dark()
        } else {
            Design::light()
        };
        let (chroma, hue) = match self.base {
            "Slate" => (0.016, 260.0),
            "Stone" => (0.008, 75.0),
            "Zinc" => (0.005, 285.0),
            _ => (0.0, 0.0),
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
        if self.accent != "Neutral" {
            let base = accent(self.accent, dark);
            design.primary.base = base;
            design.primary.foreground = if dark {
                oklch(0.145, 0.0, 0.0)
            } else {
                Color::WHITE
            };
            design.primary.border = base;
            design.primary.ring = base.alpha(0.5);
            design.primary.tint_text = base;
            design.neutral.ring = base;
            design.sidebar.primary.base = base;
            design.sidebar.primary.foreground = design.primary.foreground;
            design.sidebar.ring = base;
        }
        if self.menu == "Bold" {
            design.sidebar.accent.base = design.primary.base;
            design.sidebar.accent.foreground = design.primary.foreground;
        }
        design.chart = chart(self.chart);
        design.font_sans.regular = self.font;
        design.font_sans.medium = self.font.weight(Weight::Medium);
        design.font_sans.semibold = self.font.weight(Weight::Semibold);
        design.font_sans.bold = self.font.weight(Weight::Bold);
        design.name = format!(
            "{} {} {}",
            self.base,
            self.accent,
            if dark { "Dark" } else { "Light" }
        );
        design.seed.background = design.neutral.base.into();
        design.seed.text = design.neutral.foreground.into();
        design.seed.primary = design.primary.base.into();
        design.theme = iced::Theme::custom(design.name.clone(), design.seed);
        Theme::new(design)
    }

    /// Export the exact preview palette through public icedcn and iced APIs.
    pub fn source(&self, mode: Mode) -> String {
        let theme = self.theme(mode);
        let design = theme.design();
        let baseline = if mode == Mode::Dark {
            Design::dark()
        } else {
            Design::light()
        };
        let mut code = format!(
            "use iced::{{Font, font}};\nuse icedcn::{{Theme, core::{{Design, oklch_alpha}}}};\n\npub fn theme() -> Theme {{\n    let mut design = Design::{}();\n    design.name = {:?}.into();\n",
            if mode == Mode::Dark { "dark" } else { "light" },
            design.name,
        );
        for (name, palette, original) in [
            ("neutral", design.neutral, baseline.neutral),
            ("primary", design.primary, baseline.primary),
            ("secondary", design.secondary, baseline.secondary),
            ("muted", design.muted, baseline.muted),
            ("accent", design.accent, baseline.accent),
            ("card", design.card, baseline.card),
            ("popover", design.popover, baseline.popover),
            ("code", design.code, baseline.code),
        ] {
            for (field, color, before) in [
                ("base", palette.base, original.base),
                ("foreground", palette.foreground, original.foreground),
                ("border", palette.border, original.border),
                ("ring", palette.ring, original.ring),
                ("tint_text", palette.tint_text, original.tint_text),
            ] {
                if color != before {
                    code.push_str(&format!(
                        "    design.{name}.{field} = {};\n",
                        literal(color)
                    ));
                }
            }
        }
        for (name, color, before) in [
            ("input", design.input, baseline.input),
            (
                "sidebar.background",
                design.sidebar.background,
                baseline.sidebar.background,
            ),
            (
                "sidebar.foreground",
                design.sidebar.foreground,
                baseline.sidebar.foreground,
            ),
            (
                "sidebar.border",
                design.sidebar.border,
                baseline.sidebar.border,
            ),
            ("sidebar.ring", design.sidebar.ring, baseline.sidebar.ring),
            (
                "sidebar.primary.base",
                design.sidebar.primary.base,
                baseline.sidebar.primary.base,
            ),
            (
                "sidebar.primary.foreground",
                design.sidebar.primary.foreground,
                baseline.sidebar.primary.foreground,
            ),
            (
                "sidebar.accent.base",
                design.sidebar.accent.base,
                baseline.sidebar.accent.base,
            ),
            (
                "sidebar.accent.foreground",
                design.sidebar.accent.foreground,
                baseline.sidebar.accent.foreground,
            ),
        ] {
            if color != before {
                code.push_str(&format!("    design.{name} = {};\n", literal(color)));
            }
        }
        for (index, (color, original)) in design.chart.into_iter().zip(baseline.chart).enumerate() {
            if color != original {
                code.push_str(&format!(
                    "    design.chart[{index}] = {};\n",
                    literal(color)
                ));
            }
        }
        let family = match self.font.family {
            Family::Name(name) => format!("Font::from({name:?})"),
            family => format!("Font::with_family(font::Family::{family:?})"),
        };
        code.push_str(&format!(
            "    let font = {family};\n    design.font_sans.regular = font;\n    design.font_sans.medium = font.weight(font::Weight::Medium);\n    design.font_sans.semibold = font.weight(font::Weight::Semibold);\n    design.font_sans.bold = font.weight(font::Weight::Bold);\n    design.seed.background = design.neutral.base.into();\n    design.seed.text = design.neutral.foreground.into();\n    design.seed.primary = design.primary.base.into();\n    design.theme = iced::Theme::custom(design.name.clone(), design.seed);\n    Theme::new(design)\n}}\n\n// Load your selected font, then return or batch this Task from boot/update:\n// font::set_defaults({family}, iced::Pixels(14.0))\n"
        ));
        code
    }
}

/// Accent swatch in the selected appearance.
pub fn accent(name: &str, dark: bool) -> Color {
    let (chroma, hue) = match name {
        "Blue" => (0.19, 260.0),
        "Violet" => (0.20, 295.0),
        "Green" => (0.15, 150.0),
        "Rose" => (0.20, 15.0),
        "Orange" => (0.16, 55.0),
        _ => (0.0, 0.0),
    };
    oklch(if dark { 0.76 } else { 0.50 }, chroma, hue)
}

/// Nova chart tokens in chart-1 through chart-5 order.
fn chart(name: &str) -> [Color; 5] {
    match name {
        "Neutral" => [
            oklch(0.87, 0.0, 0.0),
            oklch(0.556, 0.0, 0.0),
            oklch(0.439, 0.0, 0.0),
            oklch(0.371, 0.0, 0.0),
            oklch(0.269, 0.0, 0.0),
        ],
        "Violet" => [
            oklch(0.811, 0.111, 293.571),
            oklch(0.606, 0.25, 292.717),
            oklch(0.541, 0.281, 293.009),
            oklch(0.491, 0.27, 292.581),
            oklch(0.432, 0.232, 292.759),
        ],
        "Green" => [
            oklch(0.871, 0.15, 154.449),
            oklch(0.723, 0.219, 149.579),
            oklch(0.627, 0.194, 149.214),
            oklch(0.527, 0.154, 150.069),
            oklch(0.448, 0.119, 151.328),
        ],
        "Rose" => [
            oklch(0.81, 0.117, 11.638),
            oklch(0.645, 0.246, 16.439),
            oklch(0.586, 0.253, 17.585),
            oklch(0.514, 0.222, 16.935),
            oklch(0.455, 0.188, 13.697),
        ],
        "Orange" => [
            oklch(0.837, 0.128, 66.29),
            oklch(0.705, 0.213, 47.604),
            oklch(0.646, 0.222, 41.116),
            oklch(0.553, 0.195, 38.402),
            oklch(0.47, 0.157, 37.304),
        ],
        _ => [
            oklch(0.809, 0.105, 251.813),
            oklch(0.623, 0.214, 259.815),
            oklch(0.546, 0.245, 262.881),
            oklch(0.488, 0.243, 264.376),
            oklch(0.424, 0.199, 265.638),
        ],
    }
}

fn literal(color: Color) -> String {
    format!(
        "oklch_alpha({:?}, {:?}, {:?}, {:?})",
        color.l, color.c, color.h, color.a
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_palettes_keep_native_and_iced_colors_in_sync() {
        for mode in [Mode::Light, Mode::Dark] {
            for base in BASES {
                for accent in ACCENTS {
                    let settings = Settings {
                        base,
                        accent,
                        menu: "Bold",
                        ..Default::default()
                    };
                    let theme = settings.theme(mode);
                    assert_eq!(theme.seed.background, theme.neutral.base.into());
                    assert_eq!(theme.seed.primary, theme.primary.base.into());
                    assert_eq!(theme.sidebar.accent.base, theme.primary.base);
                    assert_eq!(theme.sidebar.accent.foreground, theme.primary.foreground);
                    assert_eq!(theme.primary.is_dark, mode == Mode::Dark);
                }
            }
        }
    }

    #[test]
    fn chart_selection_is_exported_as_live_design_tokens() {
        for mode in [Mode::Light, Mode::Dark] {
            for name in ACCENTS {
                let settings = Settings {
                    chart: name,
                    ..Default::default()
                };
                let theme = settings.theme(mode);
                assert_eq!(theme.chart, chart(name));
                let source = settings.source(mode);
                if *name != "Blue" {
                    for (index, color) in theme.chart.into_iter().enumerate() {
                        assert!(
                            source
                                .contains(&format!("design.chart[{index}] = {};", literal(color)))
                        );
                    }
                }
                assert!(!source.contains("with_color"));
                assert_eq!(theme.primary, Settings::default().theme(mode).primary);
            }
        }
    }

    #[test]
    fn export_escapes_font_names_and_omits_unchanged_palette_fields() {
        let settings = Settings {
            font: Font::from("Custom \"Font\""),
            ..Default::default()
        };
        let source = settings.source(Mode::Dark);
        assert!(source.contains("Font::from(\"Custom \\\"Font\\\"\")"));
        assert!(!source.contains("design.neutral.base ="));
        assert!(!source.contains("chart_color"));
        assert!(!source.contains("design.chart["));
        assert!(source.contains("return or batch this Task"));
    }
}
