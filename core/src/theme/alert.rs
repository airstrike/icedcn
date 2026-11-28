//! Alert container styling
//!
//! `Style` is OKLCH-native — alert reuses container's `Style` field-for-field.
//! Variants and style tweaks apply before conversion. A [`Spec`] can also set the description
//! text color independently.

use std::ops::Add;

use super::Theme;
use super::container::Style;
use super::tweak::{Tweak, TweakSet};
use super::variant::spec;
use crate::palette::Palette;
use crate::style;
use crate::style::status;
use crate::{Background, Border, Color, radius};

// --- Variant fns -------------------------------------------------------------

/// Default alert: card surface, standard border, rounded-lg, card-foreground text.
pub fn default(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.card.base.into()),
        text_color: Some(p.card.foreground),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.neutral.border,
        },
        ..Style::default()
    }
}

/// Destructive alert: card surface, neutral border and destructive text.
pub fn destructive(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: Some(p.card.base.into()),
        text_color: Some(p.destructive.base),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: p.neutral.border,
        },
        ..Style::default()
    }
}

/// Default description color: `text-muted-foreground`.
pub fn default_description(theme: &Theme) -> Color {
    theme.muted.foreground
}

/// Destructive description color:
/// `*:data-[slot=alert-description]:text-destructive/90`.
pub fn destructive_description(theme: &Theme) -> Color {
    theme.destructive.base.scale_alpha(0.9)
}

// --- Variant unit structs ----------------------------------------------------

/// Default alert variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

pub use super::variant::Destructive;

// --- Spec --------------------------------------------------------------------

/// An alert style: a base variant function plus a sequence of tweaks
/// applied at resolve time, and the variant's description color.
#[derive(Debug, Clone)]
pub struct Spec {
    variant: fn(&Theme) -> Style,
    description: fn(&Theme) -> Color,
    tweaks: TweakSet,
}

impl Spec {
    fn variant(f: fn(&Theme) -> Style, description: fn(&Theme) -> Color) -> Spec {
        Spec {
            variant: f,
            description,
            tweaks: TweakSet::default(),
        }
    }

    fn push(mut self, t: Tweak) -> Spec {
        self.tweaks.0.push(t);
        self
    }

    /// Resolve this spec to a concrete `Style` for the given theme.
    pub fn resolve(&self, theme: &Theme) -> Style {
        let mut style = (self.variant)(theme);
        for t in &self.tweaks.0 {
            apply(t, &mut style, theme);
        }
        style
    }

    /// The color of the alert's description text.
    pub fn description(&self, theme: &Theme) -> Color {
        (self.description)(theme)
    }
}

impl From<Default> for Spec {
    fn from(_: Default) -> Spec {
        Spec::variant(default, default_description)
    }
}

impl From<Destructive> for Spec {
    fn from(_: Destructive) -> Spec {
        Spec::variant(destructive, destructive_description)
    }
}

macro_rules! variant_add {
    ($($variant:ident),* $(,)?) => {
        $(
            impl<T: Into<Tweak>> Add<T> for $variant {
                type Output = Spec;
                fn add(self, rhs: T) -> Spec {
                    Spec::from(self).push(rhs.into())
                }
            }
        )*
    };
}

variant_add!(Default);
spec!(Spec, Default);

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

// --- Resolver ----------------------------------------------------------------
//
// Alert has no status, so `Tweak::State(...)` silently drops — there's no
// hover/pressed/disabled to react to. Identical body to container.rs::apply.

fn apply(tweak: &Tweak, style: &mut Style, theme: &Theme) {
    match tweak {
        Tweak::Bg(t) => {
            style.background = Some(t.resolve(theme).into());
        }
        Tweak::Fg(t) => {
            style.text_color = Some(t.resolve(theme));
        }
        Tweak::BorderColor(t) => {
            style.border.color = t.resolve(theme);
        }
        Tweak::BorderWidth(w) => {
            style.border.width = *w;
        }
        Tweak::Radius(r) => {
            style.border.radius = *r;
        }
        Tweak::Shadow(s) => {
            style.shadow = *s;
        }
        Tweak::Opaque => {
            if let Some(Background::Color(src)) = style.background {
                let dst = theme.neutral.base;
                let composed = src.blend(dst, 1.0 - src.a).alpha(1.0);
                style.background = Some(Background::Color(composed));
            }
        }
        Tweak::Alpha(factor) => {
            if let Some(bg) = style.background.take() {
                style.background = Some(bg.scale_alpha(*factor));
            }
        }
        // No status → state combinators silently drop.
        Tweak::State(_, _) => {}
        // CSS-style cascade: tweaks with no container counterpart drop.
        Tweak::Ring { .. }
        | Tweak::Size(_)
        | Tweak::Weight(_)
        | Tweak::Family(_)
        | Tweak::LineHeight(_)
        | Tweak::Italic
        | Tweak::Align(_) => {}
    }
}

// Alert reuses iced's container widget; its intrinsic supplies the card
// surface that Tint alone can't address. Tag inner closures only contribute
// destructive text overlays; both variants keep the neutral border.

/// Alert intrinsic default in the universal Style shape — card surface,
/// 1px neutral border, LG radius, card foreground text.
pub fn intrinsic_default(theme: &Theme) -> style::Style {
    style::Style {
        fill: Some(theme.card.base.into()),
        text_color: Some(theme.card.foreground),
        border_width: Some(1.0),
        border_color: Some(theme.neutral.border),
        border_radius: Some(radius::LG.into()),
        ..style::Style::default()
    }
}

fn default_impl(_p: Palette, _s: status::Status) -> style::Style {
    style::Style::default()
}

impl From<Default> for style::StyleFn<fn(Palette, status::Status) -> style::Style> {
    fn from(_: Default) -> Self {
        style::StyleFn {
            inner: default_impl,
            tint: style::Tint::Neutral,
        }
    }
}

fn bridge<F>(sf: style::StyleFn<F>) -> iced_widget::container::StyleFn<'static, Theme>
where
    F: Fn(Palette, status::Status) -> style::Style + 'static,
{
    Box::new(move |theme| {
        let palette = theme.tinted(sf.tint);
        let user = (sf.inner)(palette, status::Status::Active);
        let default = intrinsic_default(theme);
        let merged = default + user;
        border_over_background(super::container::project_container(merged))
    })
}

// iced replaces the fill with the border at the edge of a quad. CSS paints
// the background underneath the border, so resolve that source-over color
// here in sRGB, after the Alert's background and border tweaks have resolved.
// Keep the combined alpha so translucent custom fills still blend correctly.
fn border_over_background(
    mut style: iced_widget::container::Style,
) -> iced_widget::container::Style {
    if let Some(iced_core::Background::Color(bg)) = style.background {
        let border = style.border.color;
        if border.a < 1.0 && bg.a > 0.0 {
            let remaining = bg.a * (1.0 - border.a);
            let alpha = border.a + remaining;
            style.border.color = iced_core::Color::from_rgba(
                (border.r * border.a + bg.r * remaining) / alpha,
                (border.g * border.a + bg.g * remaining) / alpha,
                (border.b * border.a + bg.b * remaining) / alpha,
                alpha,
            );
        }
    }
    style
}

// --- Bridge to iced::container::StyleFn --------------------------------------

impl<'a> From<Default> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(t: Default) -> Self {
        let sf: style::StyleFn<_> = t.into();
        bridge(sf)
    }
}

impl<'a> From<Spec> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(spec: Spec) -> Self {
        Box::new(move |t| border_over_background(spec.resolve(t).into()))
    }
}

#[cfg(test)]
mod tests {
    use super::super::tweak::{Bg, alpha, bg};
    use super::*;
    use crate::{Background, Color};

    fn bg_color(style: &Style) -> Color {
        match &style.background {
            Some(Background::Color(c)) => *c,
            other => panic!("expected solid Background::Color, got {other:?}"),
        }
    }

    /// `Default` resolves to `palette.card.base` background.
    #[test]
    fn default_uses_card_background() {
        let theme = Theme::dark();
        let resolved = default(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.card.base;
        assert!(
            (bg.l - expected.l).abs() < 1e-4
                && (bg.c - expected.c).abs() < 1e-4
                && (bg.a - expected.a).abs() < 1e-4,
            "expected card.base, got {bg:?}",
        );
    }

    /// `Bg` atom on `Default` overrides the variant background.
    #[test]
    fn bg_atom_overrides_default_variant() {
        let theme = Theme::dark();
        let spec: Spec = Default + Bg::Primary;
        let resolved = spec.resolve(&theme);
        let bg = bg_color(&resolved);
        let expected = theme.primary.base;
        assert!(
            (bg.l - expected.l).abs() < 1e-4
                && (bg.c - expected.c).abs() < 1e-4
                && (bg.a - expected.a).abs() < 1e-4,
            "expected primary.base, got {bg:?}",
        );
    }

    #[test]
    fn neutral_border_covers_the_card_in_both_variant_bridges() {
        for theme in [Theme::light(), Theme::dark()] {
            let card: iced_core::Color = theme.card.base.into();
            let border: iced_core::Color = theme.neutral.border.into();
            let styles: [iced_widget::container::StyleFn<'_, Theme>; 3] = [
                Default.into(),
                Spec::from(Default).into(),
                Spec::from(Destructive).into(),
            ];
            for style in styles {
                let painted = style(&theme);
                let expected = iced_core::Color::from_rgb(
                    border.r * border.a + card.r * (1.0 - border.a),
                    border.g * border.a + card.g * (1.0 - border.a),
                    border.b * border.a + card.b * (1.0 - border.a),
                );
                assert_eq!(painted.border.color, expected);
                assert_eq!(painted.background, Some(card.into()));
                assert_eq!(painted.border.width, 1.0);
            }
        }
    }

    #[test]
    fn border_compositing_respects_translucent_background_tweaks() {
        let theme = Theme::dark();
        let fill = iced_core::Color::from_rgba(0.2, 0.4, 0.6, 0.5);
        let border = iced_core::Color::from_rgba(0.8, 0.6, 0.4, 0.25);
        let spec = Default + bg(Color::from(fill)) + Tweak::BorderColor(Color::from(border).into());
        let style: iced_widget::container::StyleFn<'_, Theme> = spec.into();
        let painted = style(&theme);
        let expected = iced_core::Color::from_rgba(0.44, 0.48, 0.52, 0.625);
        let actual = painted.border.color;
        assert!((actual.r - expected.r).abs() < 1e-5);
        assert!((actual.g - expected.g).abs() < 1e-5);
        assert!((actual.b - expected.b).abs() < 1e-5);
        assert_eq!(actual.a, expected.a);

        let transparent: iced_widget::container::StyleFn<'_, Theme> = (Default + alpha(0.0)).into();
        assert_eq!(
            transparent(&theme).border.color,
            theme.neutral.border.into()
        );
    }
}
