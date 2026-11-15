//! Theme choices shared by the preview and the exported Rust configuration.

use iced::Font;
use iced::font::{Family, Stretch, Style, Weight};
use icedcn::Theme;
use icedcn::core::design::{Accent, Base};
use icedcn::core::{Color, Design};

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
        let design = if mode == Mode::Dark {
            Design::dark()
        } else {
            Design::light()
        };
        let mut design = design
            .with_name(self.name(mode))
            .with_base(base(self.base))
            .with_accent(preset(self.accent))
            .with_chart(preset(self.chart))
            .with_font_sans(self.font);
        if self.menu == "Bold" {
            design = design.with_menu_accent();
        }
        design.into()
    }

    /// Export the same builder recipe used by the live preview.
    pub fn source(&self, mode: Mode) -> String {
        let family = match self.font.family {
            Family::Name(name) => format!("Font::from({name:?})"),
            family => format!("Font::with_family(font::Family::{family:?})"),
        };
        let mut font = family;
        if self.font.weight != Weight::Normal {
            font.push_str(&format!(".weight(font::Weight::{:?})", self.font.weight));
        }
        if self.font.stretch != Stretch::Normal {
            font.push_str(&format!(".stretch(font::Stretch::{:?})", self.font.stretch));
        }
        if self.font.style != Style::Normal {
            font.push_str(&format!(".style(font::Style::{:?})", self.font.style));
        }
        if self.font.optical_size != Font::DEFAULT.optical_size {
            font = format!(
                "Font {{ optical_size: font::OpticalSize::{:?}, ..{font} }}",
                self.font.optical_size
            );
        }
        let mut code = format!(
            "use iced::{{Font, Task, font}};\nuse icedcn::core::design::{{Accent, Base}};\nuse icedcn::{{Design, Theme}};\n\npub fn theme() -> Theme {{\n    Design::{}()\n        .with_name({:?})\n        .with_base(Base::{:?})\n        .with_accent(Accent::{:?})\n        .with_chart(Accent::{:?})\n        .with_font_sans({font})\n",
            if mode == Mode::Dark { "dark" } else { "light" },
            self.name(mode),
            base(self.base),
            preset(self.accent),
            preset(self.chart),
        );
        if self.menu == "Bold" {
            code.push_str("        .with_menu_accent()\n");
        }
        code.push_str("        .into()\n}\n\n// Load the selected font, then return or batch font_setup() from boot/update.\npub fn font_setup<Message>() -> Task<Message> {\n    font::set_defaults(theme().font_sans.regular, 14.0)\n}\n");
        code
    }

    fn name(&self, mode: Mode) -> String {
        format!(
            "{} {} {}",
            self.base,
            self.accent,
            if mode == Mode::Dark { "Dark" } else { "Light" }
        )
    }
}

/// Accent swatch in the selected appearance.
pub fn accent(name: &str, dark: bool) -> Color {
    preset(name).color(dark)
}

fn base(name: &str) -> Base {
    match name {
        "Slate" => Base::Slate,
        "Stone" => Base::Stone,
        "Zinc" => Base::Zinc,
        _ => Base::Neutral,
    }
}

fn preset(name: &str) -> Accent {
    match name {
        "Blue" => Accent::Blue,
        "Violet" => Accent::Violet,
        "Green" => Accent::Green,
        "Rose" => Accent::Rose,
        "Orange" => Accent::Orange,
        _ => Accent::Neutral,
    }
}

#[cfg(test)]
mod export;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exported_recipe_compiles_and_reproduces_the_preview() {
        let settings = Settings {
            base: "Slate",
            accent: "Violet",
            chart: "Green",
            menu: "Bold",
            font: Font::from("A \"Quoted\" Font"),
        };
        assert_eq!(
            settings.source(Mode::Dark),
            include_str!("customize/export.rs")
        );
        assert_eq!(settings.theme(Mode::Dark), export::theme());
        let _: iced::Task<()> = export::font_setup();
    }

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
                assert_eq!(theme.chart, preset(name).chart());
                let source = settings.source(mode);
                assert!(source.contains(&format!(".with_chart(Accent::{:?})", preset(name))));
                assert!(!source.contains("with_color"));
                assert_eq!(theme.primary, Settings::default().theme(mode).primary);
            }
        }
    }

    #[test]
    fn export_escapes_font_names_and_omits_unchanged_palette_fields() {
        let settings = Settings {
            font: Font::from("Custom \"Font\""),
            chart: "Blue",
            ..Default::default()
        };
        let source = settings.source(Mode::Dark);
        assert!(source.contains("Font::from(\"Custom \\\"Font\\\"\")"));
        assert!(!source.contains("design.neutral.base ="));
        assert!(!source.contains("chart_color"));
        assert!(!source.contains("design.chart["));
        assert!(source.contains("return or batch font_setup()"));
        assert!(source.contains("pub fn font_setup<Message>() -> Task<Message>"));
        assert!(source.lines().count() <= 22);
    }
}
