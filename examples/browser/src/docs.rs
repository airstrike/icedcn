//! Light and dark palettes used by the component browser.

use icedxdy::{Color, Design, Theme};

/// Select the docs palette without changing the library's Neutral preset.
pub fn theme(dark: bool) -> Theme {
    if dark {
        return Theme::dark();
    }

    let mut design = Design::light();
    design.name = "Docs Light".into();

    // The docs use black for primary and foreground roles. Keep their
    // derived tint fields consistent for outlined and stock iced consumers.
    design.primary.base = Color::BLACK;
    design.primary.border = Color::BLACK;
    design.primary.ring = Color::BLACK.alpha(0.5);
    design.primary.tint_text = Color::BLACK;
    for palette in [
        &mut design.neutral,
        &mut design.card,
        &mut design.popover,
        &mut design.code,
    ] {
        palette.foreground = Color::BLACK;
        palette.tint_text = Color::BLACK;
    }
    design.sidebar.foreground = Color::BLACK;

    design.seed.primary = design.primary.base.into();
    design.seed.text = design.neutral.foreground.into();
    design.theme = iced::Theme::custom(design.name.clone(), design.seed);
    Theme::new(design)
}

#[cfg(test)]
mod tests {
    use super::theme;
    use crate::{Browser, Message};
    use icedxdy::{Color, Theme};

    #[test]
    fn docs_tokens_reach_tinted_and_stock_iced_consumers() {
        let docs = theme(false);
        assert_eq!(docs.primary.base, Color::BLACK);
        assert_eq!(docs.primary.border, Color::BLACK);
        assert_eq!(docs.primary.ring, Color::BLACK.alpha(0.5));
        assert_eq!(docs.primary.tint_text, Color::BLACK);
        for palette in [docs.neutral, docs.card, docs.popover, docs.code] {
            assert_eq!(palette.foreground, Color::BLACK);
            assert_eq!(palette.tint_text, Color::BLACK);
        }
        assert_eq!(docs.sidebar.foreground, Color::BLACK);
        assert_eq!(docs.seed.primary, iced::Color::BLACK);
        assert_eq!(docs.seed.text, iced::Color::BLACK);
        assert_eq!(docs.iced_theme().seed(), docs.seed);
    }

    #[test]
    fn library_neutral_and_dark_and_other_light_tokens_are_preserved() {
        let neutral = Theme::light();
        let docs = theme(false);
        assert_eq!(neutral.primary.base.l, 0.205);
        assert_eq!(neutral.neutral.foreground.l, 0.145);
        assert_eq!(theme(true), Theme::dark());

        // Revert only the documented overrides, then compare the full Design
        // so a future docs update cannot silently alter unrelated tokens.
        let mut restored = docs.design().clone();
        restored.name = neutral.name.clone();
        restored.primary = neutral.primary;
        for (palette, original) in [
            (&mut restored.neutral, neutral.neutral),
            (&mut restored.card, neutral.card),
            (&mut restored.popover, neutral.popover),
            (&mut restored.code, neutral.code),
        ] {
            assert_eq!(palette.base, original.base);
            assert_eq!(palette.border, original.border);
            assert_eq!(palette.ring, original.ring);
            palette.foreground = original.foreground;
            palette.tint_text = original.tint_text;
        }
        assert_eq!(docs.primary.foreground, neutral.primary.foreground);
        restored.sidebar.foreground = neutral.sidebar.foreground;
        restored.seed = neutral.seed;
        restored.theme = neutral.theme.clone();
        assert_eq!(&restored, neutral.design());
    }

    #[test]
    fn browser_keeps_the_docs_palette_across_repeated_theme_toggles() {
        let (mut browser, _) = Browser::new();
        for _ in 0..4 {
            let dark = browser.app.is_dark();
            assert_eq!(browser.theme(), theme(dark));
            let _ = browser.update(Message::ToggleTheme);
            assert_eq!(browser.theme(), theme(!dark));
        }
    }
}
