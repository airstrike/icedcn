//! Badge paint that retains variant identity and interaction context.

use iced_widget::container;

use super::{Default, Destructive, Ghost, Link, Outline, Secondary, Spec};
use crate::theme::Tweaked;
use crate::theme::tweak::{Trigger, Tweak, TweakSet};
use crate::{Background, Border, Color, Shadow, Theme};

/// Badge interaction context. Focus means keyboard-visible focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Status {
    pub hovered: bool,
    pub pressed: bool,
    pub focused: bool,
    pub interactive: bool,
    pub disabled: bool,
    pub invalid: bool,
}

impl Status {
    /// Remove states that cannot apply to a passive or disabled badge.
    pub fn normalized(mut self) -> Self {
        self.focused &= self.interactive && !self.disabled;
        self.pressed &= self.interactive && !self.disabled;
        self.hovered &= !self.disabled;
        self
    }
}

/// Badge paint in the theme's native color space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Style {
    pub background: Option<Background>,
    pub text_color: Option<Color>,
    pub border: Border,
    pub shadow: Shadow,
    pub snap: bool,
    /// Exterior stroke, shown only for keyboard-focused interactive badges.
    pub ring: Option<Border>,
    /// Space between the badge edge and its focus ring.
    pub ring_offset: f32,
    /// Underline text descendants, excluding SVG/icon geometry.
    pub underline: bool,
}

impl From<super::Style> for Style {
    fn from(style: super::Style) -> Self {
        Self {
            background: style.background,
            text_color: style.text_color,
            border: style.border,
            shadow: style.shadow,
            snap: style.snap,
            ..Self::default()
        }
    }
}

impl From<container::Style> for Style {
    fn from(style: container::Style) -> Self {
        Self::from(super::Style::from(style))
    }
}

/// A state-aware Badge callback. Use the widget's `state_style` builder.
pub type StyleFn<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// Accepted by Badge's `style`: variants, Specs and legacy static callbacks.
///
/// Legacy callbacks provide the same paint in every state. They do not acquire
/// guessed hover colors, underlines or rings. `state_style` is the distinct
/// entry point for a two-argument callback; the last styling builder wins.
pub trait IntoStyle<'a> {
    fn into_style(self) -> StyleFn<'a>;
}

impl<'a, F> IntoStyle<'a> for F
where
    F: Fn(&Theme) -> container::Style + 'a,
{
    fn into_style(self) -> StyleFn<'a> {
        Box::new(move |theme, _| self(theme).into())
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Variant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

macro_rules! variants {
    ($($variant:ident),+ $(,)?) => {$(
        impl<'a> IntoStyle<'a> for $variant {
            fn into_style(self) -> StyleFn<'a> {
                Box::new(|theme, status| resolve(theme, status, Variant::$variant))
            }
        }
    )+};
}
variants!(Default, Secondary, Destructive, Outline, Ghost, Link);

impl<'a, V> IntoStyle<'a> for Tweaked<V>
where
    Spec: From<V>,
{
    fn into_style(self) -> StyleFn<'a> {
        self.build::<Spec>().into_style()
    }
}

impl<'a> IntoStyle<'a> for TweakSet {
    fn into_style(self) -> StyleFn<'a> {
        Spec::from(self).into_style()
    }
}

impl<'a> IntoStyle<'a> for Spec {
    fn into_style(self) -> StyleFn<'a> {
        Box::new(move |theme, status| self.resolve_state(theme, status))
    }
}

impl Spec {
    /// Resolve Nova state defaults, then apply user tweaks in their stated order.
    pub fn resolve_state(&self, theme: &Theme, status: Status) -> Style {
        let status = status.normalized();
        let mut style = resolve(theme, status, self.kind);
        for tweak in &self.tweaks.0 {
            apply(tweak, &mut style, theme, status);
        }
        style
    }
}

fn resolve(theme: &Theme, status: Status, variant: Variant) -> Style {
    let status = status.normalized();
    let mut style: Style = match variant {
        Variant::Default => super::default(theme),
        Variant::Secondary => super::secondary(theme),
        Variant::Destructive => super::destructive(theme),
        Variant::Outline => super::outline(theme),
        Variant::Ghost => super::ghost(theme),
        Variant::Link => super::link(theme),
    }
    .into();
    // All Nova badges reserve a transparent border, including Ghost and Link.
    style.border.width = 1.0;
    if status.hovered {
        match variant {
            Variant::Default if status.interactive => {
                style.background = Some(theme.primary.base.scale_alpha(0.8).into());
            }
            Variant::Secondary if status.interactive => {
                style.background = Some(theme.secondary.base.scale_alpha(0.8).into());
            }
            Variant::Destructive if status.interactive => {
                style.background = Some(theme.destructive.base.scale_alpha(0.2).into());
            }
            Variant::Outline if status.interactive => {
                style.background = Some(theme.muted.base.into());
                style.text_color = Some(theme.muted.foreground);
            }
            Variant::Ghost => {
                let alpha = if theme.neutral.base.l < 0.5 { 0.5 } else { 1.0 };
                style.background = Some(theme.muted.base.scale_alpha(alpha).into());
                style.text_color = Some(theme.muted.foreground);
            }
            Variant::Link => style.underline = true,
            _ => {}
        }
    }
    if status.focused {
        let destructive = status.invalid || matches!(variant, Variant::Destructive);
        let color = if destructive {
            theme
                .destructive
                .base
                .scale_alpha(if theme.neutral.base.l < 0.5 { 0.4 } else { 0.2 })
        } else {
            theme.neutral.ring.scale_alpha(0.5)
        };
        style.border.color = theme.neutral.ring;
        style.ring = Some(Border {
            color,
            width: 3.0,
            radius: style.border.radius,
        });
    }
    if status.invalid {
        style.border.color = theme.destructive.base;
    }
    style
}

fn apply(tweak: &Tweak, style: &mut Style, theme: &Theme, status: Status) {
    match tweak {
        Tweak::Bg(t) => style.background = Some(t.resolve(theme).into()),
        Tweak::Fg(t) => style.text_color = Some(t.resolve(theme)),
        Tweak::BorderColor(t) => style.border.color = t.resolve(theme),
        Tweak::BorderWidth(w) => style.border.width = *w,
        Tweak::Radius(r) => style.border.radius = *r,
        Tweak::Shadow(s) => style.shadow = *s,
        Tweak::Ring {
            color,
            width,
            offset,
        } => {
            style.ring = Some(Border {
                color: color.resolve(theme),
                width: *width,
                radius: style.border.radius,
            });
            style.ring_offset = *offset;
        }
        Tweak::Opaque => {
            if let Some(Background::Color(src)) = style.background {
                style.background = Some(src.over(theme.neutral.base).into());
            }
        }
        Tweak::Alpha(factor) => {
            style.background = style.background.map(|bg| bg.scale_alpha(*factor));
        }
        Tweak::State(trigger, tweaks) => {
            let applies = match trigger {
                Trigger::Hover => status.hovered,
                Trigger::Focus => status.focused,
                Trigger::Active => status.pressed,
                Trigger::Disabled => status.disabled,
            };
            if applies {
                for tweak in &tweaks.0 {
                    apply(tweak, style, theme, status);
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{IntoStyle, Status};
    use crate::theme::badge::{self, Spec};
    use crate::theme::tweak::{Bg, Fg, Ring, Token, focus, hover};
    use crate::{Background, Theme};
    use iced_widget::container;

    #[test]
    fn passive_hover_is_variant_owned_in_both_themes() {
        for theme in [Theme::light(), Theme::dark()] {
            let rest = Status::default();
            let hovered = Status {
                hovered: true,
                ..rest
            };
            for style in [
                badge::Default.into_style(),
                badge::Secondary.into_style(),
                badge::Destructive.into_style(),
                badge::Outline.into_style(),
            ] {
                assert_eq!(style(&theme, rest), style(&theme, hovered));
            }
            let ghost = badge::Ghost.into_style();
            let after = ghost(&theme, hovered);
            let alpha = if theme.neutral.base.l < 0.5 { 0.5 } else { 1.0 };
            assert_eq!(
                after.background,
                Some(theme.muted.base.scale_alpha(alpha).into())
            );
            assert_eq!(after.text_color, Some(theme.muted.foreground));
            assert!(ghost(&theme, rest).background.is_none());
            assert!(badge::Link.into_style()(&theme, hovered).underline);
            assert!(!badge::Link.into_style()(&theme, rest).underline);
            assert_eq!(
                ghost(
                    &theme,
                    Status {
                        disabled: true,
                        ..hovered
                    }
                )
                .background,
                None
            );
        }
    }

    #[test]
    fn anchor_hover_uses_the_selected_variant_instead_of_resolved_colors() {
        let theme = Theme::light();
        let hovered = Status {
            hovered: true,
            interactive: true,
            ..Status::default()
        };
        assert_eq!(
            badge::Default.into_style()(&theme, hovered).background,
            Some(theme.primary.base.scale_alpha(0.8).into())
        );
        assert_eq!(
            badge::Secondary.into_style()(&theme, hovered).background,
            Some(theme.secondary.base.scale_alpha(0.8).into())
        );
        assert_eq!(
            badge::Destructive.into_style()(&theme, hovered).background,
            Some(theme.destructive.base.scale_alpha(0.2).into())
        );
        let outline = badge::Outline.into_style()(&theme, hovered);
        assert_eq!(outline.background, Some(theme.muted.base.into()));
        assert_eq!(outline.text_color, Some(theme.muted.foreground));
        // Equal custom fills do not change either variant's identity.
        let ghost: Spec = (badge::Ghost + Bg(Token::Primary)).into();
        assert_eq!(
            ghost
                .resolve_state(
                    &theme,
                    Status {
                        interactive: false,
                        ..hovered
                    }
                )
                .background,
            Some(theme.primary.base.into())
        );
    }

    #[test]
    fn focus_and_invalid_keep_border_width_and_require_interactive_context() {
        for theme in [Theme::light(), Theme::dark()] {
            let focused = Status {
                focused: true,
                interactive: true,
                ..Status::default()
            };
            let normal = badge::Default.into_style()(&theme, focused);
            assert_eq!(normal.border.width, 1.0);
            assert_eq!(normal.ring.expect("focus ring").width, 3.0);
            assert_eq!(
                normal.ring.expect("focus ring").color,
                theme.neutral.ring.scale_alpha(0.5)
            );
            let alpha = if theme.neutral.base.l < 0.5 { 0.4 } else { 0.2 };
            let destructive = badge::Destructive.into_style()(&theme, focused);
            assert_eq!(
                destructive.ring.expect("destructive ring").color,
                theme.destructive.base.scale_alpha(alpha)
            );
            let invalid = badge::Outline.into_style()(
                &theme,
                Status {
                    invalid: true,
                    ..focused
                },
            );
            assert_eq!(invalid.border.color, theme.destructive.base);
            assert_eq!(
                invalid.ring.expect("invalid ring").color,
                theme.destructive.base.scale_alpha(alpha)
            );
            for status in [
                Status {
                    interactive: false,
                    ..focused
                },
                Status {
                    disabled: true,
                    ..focused
                },
            ] {
                assert!(badge::Default.into_style()(&theme, status).ring.is_none());
            }
            let passive = badge::Default.into_style()(
                &theme,
                Status {
                    invalid: true,
                    ..Status::default()
                },
            );
            assert_eq!(passive.border.color, theme.destructive.base);
            assert!(passive.ring.is_none());
        }
    }

    #[test]
    fn conditional_tweaks_follow_nova_defaults_in_declared_order() {
        let theme = Theme::light();
        let spec: Spec = (badge::Ghost
            + hover(Bg(Token::Primary))
            + hover(Fg(Token::PrimaryForeground))
            + focus(Ring {
                color: Token::Destructive,
                width: 4.0,
                offset: 2.0,
            }))
        .into();
        let rest = spec.resolve_state(&theme, Status::default());
        assert_eq!(rest.background, None);
        let hovered = Status {
            hovered: true,
            interactive: true,
            ..Status::default()
        };
        assert_eq!(
            spec.resolve_state(&theme, hovered).background,
            Some(Background::Color(theme.primary.base))
        );
        let focused = spec.resolve_state(
            &theme,
            Status {
                focused: true,
                ..hovered
            },
        );
        assert_eq!(focused.ring.expect("tweaked ring").width, 4.0);
        assert_eq!(focused.ring_offset, 2.0);
        let overridden =
            Spec::from(badge::Ghost + hover(Bg(Token::Primary)) + Bg(Token::Secondary))
                .resolve_state(&theme, hovered);
        assert_eq!(overridden.background, Some(theme.secondary.base.into()));
    }

    #[test]
    fn legacy_callback_has_explicit_static_semantics() {
        let legacy: container::StyleFn<'_, Theme> =
            Box::new(|_| container::Style::default().background(iced_core::Color::BLACK));
        let paint = legacy.into_style();
        let theme = Theme::light();
        let rest = paint(&theme, Status::default());
        assert_eq!(
            rest,
            paint(
                &theme,
                Status {
                    hovered: true,
                    focused: true,
                    interactive: true,
                    invalid: true,
                    ..Status::default()
                }
            )
        );
        assert!(rest.ring.is_none());
        assert!(!rest.underline);
    }
}
