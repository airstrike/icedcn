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
            chart: "Blue",
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
        let family = match self.font.family {
            Family::Name(name) => format!("Font::from({name:?})"),
            family => format!("Font::with_family(font::Family::{family:?})"),
        };
        code.push_str(&format!(
            "    let font = {family};\n    design.font_sans.regular = font;\n    design.font_sans.medium = font.weight(font::Weight::Medium);\n    design.font_sans.semibold = font.weight(font::Weight::Semibold);\n    design.font_sans.bold = font.weight(font::Weight::Bold);\n    design.seed.background = design.neutral.base.into();\n    design.seed.text = design.neutral.foreground.into();\n    design.seed.primary = design.primary.base.into();\n    design.theme = iced::Theme::custom(design.name.clone(), design.seed);\n    Theme::new(design)\n}}\n\n// Load your selected font, then return or batch this Task from boot/update:\n// font::set_defaults({family}, iced::Pixels(14.0))\n"
        ));
        let chart = accent(self.chart, mode == Mode::Dark);
        code.push_str(&format!(
            "\n// Desktop: .with_color(chart_color())\n// Mobile: .with_color(chart_color().scale_alpha(0.5))\npub fn chart_color() -> iced::Color {{\n    {}.into()\n}}\n",
            literal(chart),
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
    fn export_escapes_font_names_and_omits_unchanged_palette_fields() {
        let settings = Settings {
            font: Font::from("Custom \"Font\""),
            ..Default::default()
        };
        let source = settings.source(Mode::Dark);
        assert!(source.contains("Font::from(\"Custom \\\"Font\\\"\")"));
        assert!(!source.contains("design.neutral.base ="));
        assert!(source.contains("scale_alpha(0.5)"));
        assert!(source.contains("return or batch this Task"));
    }
}
