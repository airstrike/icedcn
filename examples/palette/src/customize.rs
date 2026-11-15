//! Theme choices shared by the preview and the exported Rust configuration.

use iced::font::{Family, Stretch, Style, Weight};
use iced::{Font, theme};
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

/// Appearance policies offered by the Create builder.
pub const APPEARANCES: &[&str] = &["System preference", "Always light", "Always dark"];

/// How a design responds to the host's current appearance.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Appearance {
    #[default]
    System,
    Light,
    Dark,
}

impl Appearance {
    pub fn label(self) -> &'static str {
        match self {
            Self::System => APPEARANCES[0],
            Self::Light => APPEARANCES[1],
            Self::Dark => APPEARANCES[2],
        }
    }

    pub fn resolve(self, mode: Mode) -> Mode {
        match self {
            Self::System => mode,
            Self::Light => Mode::Light,
            Self::Dark => Mode::Dark,
        }
    }
}

/// Customizations that can be represented by icedcn's public design API.
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub appearance: Appearance,
    pub base: &'static str,
    pub accent: &'static str,
    pub chart: &'static str,
    pub menu: &'static str,
    pub font: Font,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            appearance: Appearance::System,
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
        let design = Design::from(theme::Mode::from(self.appearance.resolve(mode)));
        let mut design = design
            .with_name(self.name())
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
    pub fn source(&self) -> String {
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
        let (font, imports) = match self.font.family {
            Family::Name(name) if self.font == Font::from(name) => {
                (format!("{name:?}"), "Task, font")
            }
            _ => (font, "Font, Task, font"),
        };
        let (parameter, mode) = match self.appearance {
            Appearance::System => ("mode", "mode"),
            Appearance::Light => ("_mode", "Mode::Light"),
            Appearance::Dark => ("_mode", "Mode::Dark"),
        };
        let mut code = format!(
            "use iced::theme::Mode;\nuse iced::{{{imports}}};\nuse icedcn::core::design::{{Accent, Base}};\nuse icedcn::{{Design, Theme}};\n\npub fn theme({parameter}: Mode) -> Theme {{\n    Design::from({mode})\n        .with_name({:?})\n        .with_base(Base::{:?})\n        .with_accent(Accent::{:?})\n        .with_chart(Accent::{:?})\n        .with_font_sans({font})\n",
            self.name(),
            base(self.base),
            preset(self.accent),
            preset(self.chart),
        );
        if self.menu == "Bold" {
            code.push_str("        .with_menu_accent()\n");
        }
        code.push_str(&format!("        .into()\n}}\n\n// Pass the system preference or your app's chosen mode to theme(mode).\n// Load the selected font, then return or batch font_setup() from boot/update.\npub fn font_setup<Message>() -> Task<Message> {{\n    font::set_defaults({font}.into(), 14.0)\n}}\n"));
        code
    }

    fn name(&self) -> String {
        if self.base == self.accent {
            self.base.to_owned()
        } else {
            format!("{} {}", self.base, self.accent)
        }
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
mod dark;
#[cfg(test)]
mod export;
#[cfg(test)]
mod light;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exported_recipe_compiles_and_reproduces_the_preview() {
        let mut settings = Settings {
            appearance: Appearance::System,
            base: "Slate",
            accent: "Violet",
            chart: "Green",
            menu: "Bold",
            font: Font::from("A \"Quoted\" Font"),
        };
        let recipes: [(Appearance, &str, fn(theme::Mode) -> Theme); 3] = [
            (
                Appearance::System,
                include_str!("customize/export.rs"),
                export::theme,
            ),
            (
                Appearance::Light,
                include_str!("customize/light.rs"),
                light::theme,
            ),
            (
                Appearance::Dark,
                include_str!("customize/dark.rs"),
                dark::theme,
            ),
        ];
        for (appearance, source, factory) in recipes {
            settings.appearance = appearance;
            assert_eq!(settings.source(), source);
            for mode in [Mode::Light, Mode::Dark] {
                assert_eq!(settings.theme(mode), factory(mode.into()));
            }
            assert_eq!(settings.theme(Mode::Light), factory(theme::Mode::None));
        }
        let _: iced::Task<()> = export::font_setup();
        let _: iced::Task<()> = light::font_setup();
        let _: iced::Task<()> = dark::font_setup();
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
                let source = settings.source();
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
        let source = settings.source();
        assert!(source.contains(".with_font_sans(\"Custom \\\"Font\\\"\")"));
        assert!(!source.contains("use iced::{Font"));
        assert!(!source.contains("design.neutral.base ="));
        assert!(!source.contains("chart_color"));
        assert!(!source.contains("design.chart["));
        assert!(source.contains("return or batch font_setup()"));
        assert!(source.contains("pub fn font_setup<Message>() -> Task<Message>"));
        assert!(source.lines().count() <= 22);
    }
}
