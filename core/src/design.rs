//! Design configuration — the superset of Theme.
//!
//! Holds the full palette field set (per-role + surfaces + sidebar),
//! the derived iced Theme, fonts, and identity.

mod accent;
mod base;

pub use accent::Accent;
pub use base::Base;

use iced_core::Font;
use iced_core::font::Weight;
use iced_core::theme::Mode;
use iced_core::theme::palette::Seed;

use crate::color::{Color, oklch, oklch_alpha};
use crate::palette::{Palette, SidebarPair, SidebarPalette};

/// Font family with weight variants
#[derive(Debug, Clone, PartialEq)]
pub struct FontFamily {
    pub regular: Font,
    pub medium: Font,
    pub semibold: Font,
    pub bold: Font,
}

impl FontFamily {
    pub fn from_name(name: &'static str) -> Self {
        Font::from(name).into()
    }
}

impl From<Font> for FontFamily {
    fn from(font: Font) -> Self {
        Self {
            regular: font,
            medium: font.weight(Weight::Medium),
            semibold: font.weight(Weight::Semibold),
            bold: font.weight(Weight::Bold),
        }
    }
}

impl From<&'static str> for FontFamily {
    fn from(name: &'static str) -> Self {
        Self::from_name(name)
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::from_name("Inter Tight")
    }
}

/// Complete design configuration — per-role palettes, surfaces, sidebar,
/// fonts, and the derived iced Theme.
#[derive(Debug, Clone, PartialEq)]
pub struct Design {
    pub name: String,

    // Intent palettes — selectable via `Tint`.
    pub neutral: Palette,
    pub primary: Palette,
    pub secondary: Palette,
    pub muted: Palette,
    pub accent: Palette,
    pub destructive: Palette,
    pub info: Palette,
    pub success: Palette,
    pub warning: Palette,

    // Surface palettes — not Tint-eligible; widgets read them directly.
    pub card: Palette,
    pub popover: Palette,
    pub code: Palette,

    // Sidebar context — deferred reshape; stays as its own struct.
    pub sidebar: SidebarPalette,

    /// Input-field border color. Distinct from `neutral.border` in dark
    /// mode (15% vs 10% alpha) — input fields need higher contrast.
    pub input: Color,

    /// Chart-1 through chart-5 tokens, independent of control intent colors.
    pub chart: [Color; 5],

    pub seed: Seed,
    pub theme: iced_core::Theme,
    pub font_sans: FontFamily,
    pub font_mono: Font,
}

impl Design {
    /// Light mode design (Neutral theme).
    pub fn light() -> Self {
        // Structural greys — chrome borders + focus ring.
        let neutral_border = oklch(0.922, 0.0, 0.0);
        let neutral_ring = oklch(0.708, 0.0, 0.0);

        let neutral = Palette {
            base: oklch(1.0, 0.0, 0.0),
            foreground: oklch(0.145, 0.0, 0.0),
            border: neutral_border,
            ring: neutral_ring,
            tint_text: oklch(0.145, 0.0, 0.0),
            is_dark: false,
        };

        let primary = tinted_palette(oklch(0.205, 0.0, 0.0), oklch(0.985, 0.0, 0.0), false);
        let secondary = tinted_palette(oklch(0.97, 0.0, 0.0), oklch(0.205, 0.0, 0.0), false);
        let muted = tinted_palette(oklch(0.97, 0.0, 0.0), oklch(0.556, 0.0, 0.0), false);
        let accent = tinted_palette(oklch(0.97, 0.0, 0.0), oklch(0.205, 0.0, 0.0), false);
        let destructive =
            tinted_palette(oklch(0.577, 0.245, 27.325), oklch(0.97, 0.01, 17.0), false);
        let info = tinted_palette(oklch(0.55, 0.20, 245.0), oklch(0.985, 0.0, 0.0), false);
        let success = tinted_palette(oklch(0.58, 0.18, 150.0), oklch(0.985, 0.0, 0.0), false);
        let warning = tinted_palette(oklch(0.72, 0.16, 75.0), oklch(0.205, 0.0, 0.0), false);

        let card = surface_palette(
            oklch(1.0, 0.0, 0.0),
            oklch(0.145, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            false,
        );
        let popover = surface_palette(
            oklch(1.0, 0.0, 0.0),
            oklch(0.145, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            false,
        );
        let code = surface_palette(
            oklch(0.98, 0.0, 0.0),
            oklch(0.145, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            false,
        );

        let sidebar = SidebarPalette {
            background: oklch(0.985, 0.0, 0.0),
            foreground: oklch(0.145, 0.0, 0.0),
            primary: SidebarPair {
                base: oklch(0.205, 0.0, 0.0),
                foreground: oklch(0.985, 0.0, 0.0),
            },
            accent: SidebarPair {
                base: oklch(0.97, 0.0, 0.0),
                foreground: oklch(0.205, 0.0, 0.0),
            },
            border: oklch(0.922, 0.0, 0.0),
            ring: oklch(0.708, 0.0, 0.0),
        };

        let input = oklch(0.922, 0.0, 0.0);

        build(
            "Light",
            neutral,
            primary,
            secondary,
            muted,
            accent,
            destructive,
            info,
            success,
            warning,
            card,
            popover,
            code,
            sidebar,
            input,
        )
    }

    /// Dark mode design (Neutral theme).
    pub fn dark() -> Self {
        let neutral_border = oklch_alpha(1.0, 0.0, 0.0, 0.1);
        let neutral_ring = oklch(0.556, 0.0, 0.0);

        let neutral = Palette {
            base: oklch(0.145, 0.0, 0.0),
            foreground: oklch(0.985, 0.0, 0.0),
            border: neutral_border,
            ring: neutral_ring,
            tint_text: oklch(0.985, 0.0, 0.0),
            is_dark: true,
        };

        let primary = tinted_palette(oklch(0.922, 0.0, 0.0), oklch(0.205, 0.0, 0.0), true);
        let secondary = tinted_palette(oklch(0.269, 0.0, 0.0), oklch(0.985, 0.0, 0.0), true);
        let muted = tinted_palette(oklch(0.269, 0.0, 0.0), oklch(0.708, 0.0, 0.0), true);
        let accent = tinted_palette(oklch(0.371, 0.0, 0.0), oklch(0.985, 0.0, 0.0), true);
        let destructive =
            tinted_palette(oklch(0.704, 0.191, 22.216), oklch(0.58, 0.22, 27.0), true);
        let info = tinted_palette(oklch(0.70, 0.16, 245.0), oklch(0.205, 0.0, 0.0), true);
        let success = tinted_palette(oklch(0.72, 0.16, 150.0), oklch(0.205, 0.0, 0.0), true);
        let warning = tinted_palette(oklch(0.80, 0.15, 80.0), oklch(0.205, 0.0, 0.0), true);

        let card = surface_palette(
            oklch(0.205, 0.0, 0.0),
            oklch(0.985, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            true,
        );
        let popover = surface_palette(
            oklch(0.205, 0.0, 0.0),
            oklch(0.985, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            true,
        );
        let code = surface_palette(
            oklch(0.2, 0.0, 0.0),
            oklch(0.708, 0.0, 0.0),
            neutral_border,
            neutral_ring,
            true,
        );

        let sidebar = SidebarPalette {
            background: oklch(0.205, 0.0, 0.0),
            foreground: oklch(0.985, 0.0, 0.0),
            primary: SidebarPair {
                base: oklch(0.488, 0.243, 264.376),
                foreground: oklch(0.985, 0.0, 0.0),
            },
            accent: SidebarPair {
                base: oklch(0.269, 0.0, 0.0),
                foreground: oklch(0.985, 0.0, 0.0),
            },
            border: oklch_alpha(1.0, 0.0, 0.0, 0.1),
            ring: oklch(0.439, 0.0, 0.0),
        };

        let input = oklch_alpha(1.0, 0.0, 0.0, 0.15);

        build(
            "Dark",
            neutral,
            primary,
            secondary,
            muted,
            accent,
            destructive,
            info,
            success,
            warning,
            card,
            popover,
            code,
            sidebar,
            input,
        )
    }

    /// Name this design and its derived iced theme.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Apply a neutral surface family without changing accent or chart colors.
    pub fn with_base(mut self, base: Base) -> Self {
        base.apply(&mut self);
        self
    }

    /// Apply an accent to controls, focus rings and sidebar primary colors.
    pub fn with_accent(mut self, accent: Accent) -> Self {
        let dark = self.neutral.is_dark;
        if accent != Accent::Neutral {
            let base = accent.color(dark);
            self.primary.base = base;
            self.primary.foreground = if dark {
                oklch(0.145, 0.0, 0.0)
            } else {
                Color::WHITE
            };
            self.primary.border = base;
            self.primary.ring = base.alpha(0.5);
            self.primary.tint_text = base;
            self.neutral.ring = base;
            self.sidebar.primary.base = base;
            self.sidebar.primary.foreground = self.primary.foreground;
            self.sidebar.ring = base;
        }
        self
    }

    /// Set chart colors from a named family or an explicit five-color array.
    pub fn with_chart(mut self, colors: impl Into<[Color; 5]>) -> Self {
        self.chart = colors.into();
        self
    }

    /// Use primary colors for selected sidebar items.
    pub fn with_menu_accent(mut self) -> Self {
        self.sidebar.accent.base = self.primary.base;
        self.sidebar.accent.foreground = self.primary.foreground;
        self
    }

    /// Set a font family, deriving weight variants when given a font or family name.
    pub fn with_font_sans(mut self, font: impl Into<FontFamily>) -> Self {
        self.font_sans = font.into();
        self
    }

    pub fn with_font_mono(mut self, font: impl Into<Font>) -> Self {
        self.font_mono = font.into();
        self
    }
}

/// Resolve iced's current preference; an unspecified preference uses light mode.
impl From<Mode> for Design {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Dark => Self::dark(),
            Mode::Light | Mode::None => Self::light(),
        }
    }
}

impl Default for Design {
    fn default() -> Self {
        Self::light()
    }
}

// Transparent-fill widgets use the role’s base color for text, keeping outlined destructive
// controls legible against the underlying surface.
fn tinted_palette(base: Color, foreground: Color, is_dark: bool) -> Palette {
    Palette {
        base,
        foreground,
        border: base,
        ring: base.alpha(0.5),
        tint_text: base,
        is_dark,
    }
}

// Surface palettes (card, popover, code) inherit neutral's border + ring.
// `tint_text = foreground` — surfaces aren't role tints; the readable
// text color over them is `foreground` (the page text).
fn surface_palette(
    base: Color,
    foreground: Color,
    border: Color,
    ring: Color,
    is_dark: bool,
) -> Palette {
    Palette {
        base,
        foreground,
        border,
        ring,
        tint_text: foreground,
        is_dark,
    }
}

#[allow(clippy::too_many_arguments)]
fn build(
    name: &'static str,
    neutral: Palette,
    primary: Palette,
    secondary: Palette,
    muted: Palette,
    accent: Palette,
    destructive: Palette,
    info: Palette,
    success: Palette,
    warning: Palette,
    card: Palette,
    popover: Palette,
    code: Palette,
    sidebar: SidebarPalette,
    input: Color,
) -> Design {
    let seed = Seed {
        background: neutral.base.into(),
        text: neutral.foreground.into(),
        primary: primary.base.into(),
        success: accent.base.into(),
        danger: destructive.base.into(),
        warning: secondary.base.into(),
    };
    let name = name.to_string();
    let theme = iced_core::Theme::custom(name.clone(), seed);
    Design {
        name,
        neutral,
        primary,
        secondary,
        muted,
        accent,
        destructive,
        info,
        success,
        warning,
        card,
        popover,
        code,
        sidebar,
        input,
        chart: Accent::Blue.chart(),
        seed,
        theme,
        font_sans: FontFamily::default(),
        font_mono: Font::MONOSPACE,
    }
}

#[cfg(test)]
mod tests {
    use super::{Accent, Base, Design, FontFamily};
    use crate::{Theme, oklch};
    use iced_core::{Font, color, font};

    #[test]
    fn iced_mode_selects_the_design_with_a_light_fallback() {
        use iced_core::theme::Mode;

        assert_eq!(Design::from(Mode::Light), Design::light());
        assert_eq!(Design::from(Mode::Dark), Design::dark());
        assert_eq!(Design::from(Mode::None), Design::light());
    }

    #[test]
    fn presets_keep_surface_accent_chart_and_font_choices_independent() {
        for design in [Design::light(), Design::dark()] {
            let dark = design.neutral.is_dark;
            let initial_chart = design.chart;
            let surface = design.with_base(Base::Slate);
            assert_eq!(surface.chart, initial_chart);
            assert_eq!(surface.muted.base.c, 0.016);
            let font = Font::from("Selected Font").style(font::Style::Italic);
            let theme: Theme = surface
                .with_name("Custom")
                .with_accent(Accent::Violet)
                .with_chart(Accent::Green)
                .with_font_sans(font)
                .with_menu_accent()
                .into();
            assert_eq!(theme.primary.base, Accent::Violet.color(dark));
            assert_eq!(theme.chart, Accent::Green.chart());
            assert_eq!(theme.sidebar.accent.base, theme.primary.base);
            assert_eq!(theme.sidebar.accent.foreground, theme.primary.foreground);
            assert_eq!(theme.font_sans.regular, font);
            assert_eq!(theme.font_sans.bold, font.weight(font::Weight::Bold));
            assert_eq!(theme.seed.background, theme.neutral.base.into());
            assert_eq!(theme.seed.text, theme.neutral.foreground.into());
            assert_eq!(theme.seed.primary, theme.primary.base.into());
            assert_eq!(theme.iced_theme().seed(), theme.seed);
        }
    }

    #[test]
    fn finalization_preserves_explicit_colors_fonts_and_other_seed_roles() {
        let tokens = [oklch(0.7, 0.1, 150.0); 5];
        let fonts = FontFamily {
            regular: Font::MONOSPACE,
            medium: Font::from("Custom Medium"),
            semibold: Font::from("Custom Semibold"),
            bold: Font::from("Custom Bold"),
        };
        let mut design = Design::light()
            .with_chart(tokens)
            .with_font_sans(fonts.clone());
        design.primary.base = oklch(0.6, 0.15, 50.0);
        design.seed.success = color!(0xff0077);
        let theme: Theme = design.into();
        assert_eq!(theme.chart, tokens);
        assert_eq!(theme.font_sans, fonts);
        assert_eq!(theme.seed.success, color!(0xff0077));
        assert_eq!(theme.seed.primary, theme.primary.base.into());
    }
}
