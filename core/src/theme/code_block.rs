//! Code Block panel styling.
//!
//! `.style(Destructive)` picks the destructive panel, and tweaks compose on
//! either variant: `Destructive + Bg(Token::Muted)`, or a bare
//! `Bg(Token::Muted)` on the default panel.

use std::ops::Add;

use super::Theme;
use super::container::{self, Style};
use super::tweak::{Tweak, TweakSet};
use super::variant::spec;
use crate::{Border, radius};

/// Default panel: `border border-input text-foreground dark:bg-input/30`.
pub fn default(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.neutral.foreground),
        background: theme
            .neutral
            .is_dark
            .then(|| theme.input.scale_alpha(0.3).into()),
        border: Border {
            width: 1.0,
            radius: radius::LG.into(),
            color: theme.input,
        },
        ..Style::default()
    }
}

/// Destructive panel: `border-destructive/30 text-destructive`
/// (`dark:border-destructive/40`).
pub fn destructive(theme: &Theme) -> Style {
    container::destructive(theme)
}

/// The default panel (`variant="default"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Default;

pub use super::variant::Destructive;

/// A panel style: a base variant plus tweaks applied at resolve time.
#[derive(Debug, Clone)]
pub struct Spec {
    variant: fn(&Theme) -> Style,
    focus_border: bool,
    tweaks: TweakSet,
}

impl Spec {
    /// A spec painted by `variant`: the base of a variant of your own.
    pub fn new(variant: fn(&Theme) -> Style) -> Spec {
        Spec {
            variant,
            focus_border: false,
            tweaks: TweakSet::default(),
        }
    }

    fn push(mut self, tweak: Tweak) -> Spec {
        self.tweaks.0.push(tweak);
        self
    }

    /// Resolve this spec to a concrete `Style` for the given theme.
    pub fn resolve(&self, theme: &Theme) -> Style {
        let mut style = (self.variant)(theme);
        for tweak in &self.tweaks.0 {
            container::apply(tweak, &mut style, theme);
        }
        style
    }

    /// Whether keyboard focus in the content recolors the border to the ring.
    /// Only the default variant does (`has-[:focus-visible]:border-ring`).
    pub fn focus_border(&self) -> bool {
        self.focus_border
    }
}

impl core::default::Default for Spec {
    fn default() -> Spec {
        Spec::from(Default)
    }
}

impl From<Default> for Spec {
    fn from(_: Default) -> Spec {
        Spec {
            focus_border: true,
            ..Spec::new(default)
        }
    }
}

impl From<Destructive> for Spec {
    fn from(_: Destructive) -> Spec {
        Spec::new(destructive)
    }
}

impl<T: Into<Tweak>> Add<T> for Default {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        Spec::from(self).push(rhs.into())
    }
}

impl<T: Into<Tweak>> Add<T> for Spec {
    type Output = Spec;
    fn add(self, rhs: T) -> Spec {
        self.push(rhs.into())
    }
}

spec!(Spec, Default);

#[cfg(test)]
mod tests {
    use super::super::tweak::{Bg, Token};
    use super::*;

    #[test]
    fn variants_and_tweaks_resolve_on_the_panel() {
        let theme = Theme::dark();
        let muted = Some(theme.muted.base.into());
        let tweaked: Spec = (Destructive + Bg(Token::Muted)).into();
        let tweaked = tweaked.resolve(&theme);
        assert_eq!(tweaked.background, muted);
        assert_eq!(tweaked.text_color, Some(theme.destructive.base));
        let bare = Spec::from(Bg(Token::Muted));
        assert!(bare.focus_border());
        assert_eq!(bare.resolve(&theme).border, default(&theme).border);
        assert_eq!(bare.resolve(&theme).background, muted);
        assert!(!Spec::from(Destructive).focus_border());
    }
}
