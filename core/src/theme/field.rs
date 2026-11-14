//! Card surfaces for fields. [`Card`] has a neutral border and transparent background.
//! [`CardActive`] uses the primary color to highlight a checked control.

use super::Theme;
use super::container::Style;
use crate::{Border, radius};

// --- Variant fns -------------------------------------------------------------

/// Transparent card with a one-pixel border and rounded corners.
pub fn card(theme: &Theme) -> Style {
    let p = theme;
    Style {
        background: None,
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: p.neutral.border,
        },
        ..Style::default()
    }
}

/// Checked field card with a primary-tinted background and border. Dark themes use a stronger
/// fill and a subtler border.
pub fn card_active(theme: &Theme) -> Style {
    use crate::Background;
    let p = theme;
    let dark = p.neutral.base.l < 0.5;
    let (bg_alpha, border_alpha) = if dark { (0.10, 0.20) } else { (0.05, 0.30) };
    Style {
        background: Some(Background::Color(p.primary.base.alpha(bg_alpha))),
        border: Border {
            radius: radius::MD.into(),
            width: 1.0,
            color: p.primary.base.alpha(border_alpha),
        },
        ..Style::default()
    }
}

// --- Variant unit structs ----------------------------------------------------

/// Inactive field card container variant.
///
/// Renders a 1px `palette.border`-colored border with transparent background
/// and `rounded-lg` (`radius::MD`) corners. Use when the inner control is
/// unchecked / inactive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card;

/// Active field card container variant.
///
/// Renders `border-primary/30` + `bg-primary/5` chrome. Use when the inner
/// control is checked / active. Caller is responsible for selecting between
/// `Card` and `CardActive` — iced has no `data-checked` cascade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardActive;

// --- Bridge to iced::container::StyleFn --------------------------------------

impl<'a> From<Card> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: Card) -> Self {
        Box::new(|t| card(t).into())
    }
}

impl<'a> From<CardActive> for iced_widget::container::StyleFn<'a, Theme> {
    fn from(_: CardActive) -> Self {
        Box::new(|t| card_active(t).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Inactive card: transparent bg, 1px border using `palette.border`.
    #[test]
    fn card_uses_palette_border() {
        let theme = Theme::dark();
        let p = &theme;
        let style = card(&theme);
        assert!(
            style.background.is_none(),
            "inactive card must have no background",
        );
        assert!(
            (style.border.width - 1.0).abs() < 1e-4,
            "card must have a 1px border",
        );
        let bc = style.border.color;
        let expected = p.neutral.border;
        assert!(
            (bc.l - expected.l).abs() < 1e-3
                && (bc.c - expected.c).abs() < 1e-3
                && (bc.a - expected.a).abs() < 1e-3,
            "inactive card border must match palette.border. got {bc:?}, expected {expected:?}",
        );
    }

    /// Light active cards use a 5% fill and a 30% border.
    #[test]
    fn card_active_light_alphas() {
        use crate::Background;
        let theme = Theme::light();
        let p = &theme;
        let style = card_active(&theme);
        let ep = p.primary.base;
        let bc = style.border.color;
        assert!(
            (bc.l - ep.l).abs() < 1e-3 && (bc.c - ep.c).abs() < 1e-3,
            "active card border must share primary hue, got {bc:?}",
        );
        assert!(
            (bc.a - 0.30).abs() < 1e-3,
            "light-mode border alpha must be 0.30, got {}",
            bc.a,
        );
        let Some(Background::Color(bg)) = style.background else {
            panic!("expected Background::Color, got {:?}", style.background);
        };
        assert!(
            (bg.l - ep.l).abs() < 1e-3 && (bg.c - ep.c).abs() < 1e-3,
            "active card bg must share primary hue, got {bg:?}",
        );
        assert!(
            (bg.a - 0.05).abs() < 1e-3,
            "light-mode bg alpha must be 0.05, got {}",
            bg.a,
        );
    }

    /// Dark active cards use a 10% fill and a 20% border.
    #[test]
    fn card_active_dark_alphas() {
        use crate::Background;
        let theme = Theme::dark();
        let p = &theme;
        let style = card_active(&theme);
        let ep = p.primary.base;
        let bc = style.border.color;
        assert!(
            (bc.a - 0.20).abs() < 1e-3,
            "dark-mode border alpha must be 0.20, got {}",
            bc.a,
        );
        let Some(Background::Color(bg)) = style.background else {
            panic!("expected Background::Color, got {:?}", style.background);
        };
        assert!(
            (bg.a - 0.10).abs() < 1e-3,
            "dark-mode bg alpha must be 0.10, got {}",
            bg.a,
        );
        // Sanity: hue is still primary in dark mode.
        assert!(
            (bg.l - ep.l).abs() < 1e-3 && (bg.c - ep.c).abs() < 1e-3,
            "dark-mode bg must share primary hue, got {bg:?}",
        );
    }
}
