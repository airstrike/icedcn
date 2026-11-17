//! Button's lossless style boundary. Stock callbacks remain compatibility inputs;
//! variant identity is retained only when the caller supplies a variant or Spec.

use iced_core::Border;
use iced_widget::button;

use super::super::Tweaked;
use super::super::tweak::{Trigger, Tweak, TweakSet};
use super::{Spec, Theme};
use crate::palette::Palette;
use crate::style;

/// Simultaneous visual states, including keyboard-visible focus.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Status {
    pub hovered: bool,
    pub pressed: bool,
    pub focused: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub expanded: bool,
    pub anchor: bool,
    pub small: bool,
}

impl Status {
    pub fn stock(self) -> button::Status {
        if self.disabled {
            button::Status::Disabled
        } else if self.pressed {
            button::Status::Pressed
        } else if self.hovered {
            button::Status::Hovered
        } else {
            button::Status::Active
        }
    }

    fn universal(self) -> style::Status {
        if self.disabled {
            style::Status::Disabled
        } else if self.invalid {
            style::Status::Invalid
        } else if self.pressed {
            style::Status::Pressed
        } else if self.focused {
            style::Status::Focused
        } else if self.hovered {
            style::Status::Hovered
        } else {
            style::Status::Active
        }
    }
}

/// Resolved paint plus decorations that iced's stock Button style cannot carry.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Style {
    pub paint: button::Style,
    pub ring: Option<Border>,
    pub ring_offset: f32,
    pub underline: bool,
}

/// A Button style callback after conversion.
pub type StyleFn<'a, Theme = super::Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// Convert a supported public style without erasing Button decorations.
///
/// Stock functions/closures retain their stock fields exactly and add no inferred
/// variant, focus ring or underline. `map_style` only transforms those stock fields.
pub trait IntoStyle<'a> {
    /// Whether this input uses Nova's border-box geometry.
    fn uses_border_box(&self) -> bool {
        true
    }
    fn into_style(self) -> StyleFn<'a>;
}

impl<'a, F> IntoStyle<'a> for F
where
    F: Fn(&Theme, button::Status) -> button::Style + 'a,
{
    fn uses_border_box(&self) -> bool {
        false
    }
    fn into_style(self) -> StyleFn<'a> {
        Box::new(move |theme, status| Style {
            paint: self(theme, status.stock()),
            ..Style::default()
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Variant {
    Default,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
    Sidebar,
    SidebarActive,
    Custom,
}

macro_rules! variants {
    ($($name:ident),+ $(,)?) => {$(
        impl<'a> IntoStyle<'a> for super::$name {
            fn into_style(self) -> StyleFn<'a> {
                Spec::from(self).into_style()
            }
        }
    )+};
}
variants!(Default, Secondary, Outline, Ghost, Destructive, Link);

impl<'a> IntoStyle<'a> for super::Sidebar {
    fn uses_border_box(&self) -> bool {
        false
    }
    fn into_style(self) -> StyleFn<'a> {
        let stock: button::StyleFn<'a, Theme> = self.into();
        stock.into_style()
    }
}
impl<'a> IntoStyle<'a> for super::SidebarActive {
    fn uses_border_box(&self) -> bool {
        false
    }
    fn into_style(self) -> StyleFn<'a> {
        let stock: button::StyleFn<'a, Theme> = self.into();
        stock.into_style()
    }
}

impl<'a> IntoStyle<'a> for Spec {
    fn uses_border_box(&self) -> bool {
        !matches!(
            self.kind,
            Variant::Custom | Variant::Sidebar | Variant::SidebarActive
        )
    }

    fn into_style(self) -> StyleFn<'a> {
        Box::new(move |theme, status| self.resolve_state(theme, status))
    }
}

impl Spec {
    /// Resolve Nova defaults before applying the caller's ordered tweaks.
    pub fn resolve_state(&self, theme: &Theme, status: Status) -> Style {
        if matches!(
            self.kind,
            Variant::Custom | Variant::Sidebar | Variant::SidebarActive
        ) {
            return Style {
                paint: self.resolve(theme, status.stock()).into(),
                ..Style::default()
            };
        }
        let dark = super::is_dark(theme);
        // Press changes geometry, not the hover surface's opacity.
        let base_status = if status.hovered && !status.disabled {
            button::Status::Hovered
        } else {
            button::Status::Active
        };
        let mut paint = (self.variant)(theme, base_status);
        if matches!(self.kind, Variant::Link) {
            // Nova decorates the text; it does not dim it on hover.
            paint.text_color = theme.primary.base;
        }
        paint.border.width = 1.0;
        // Browsers paint border boxes on device pixels; a centered button lands on
        // half points, where an unsnapped 1px border smears into two faint rows.
        paint.snap = true;
        paint.border.radius = if status.small { 8.0 } else { 10.0 }.into();
        if status.anchor && status.hovered && matches!(self.kind, Variant::Default) {
            paint.background = Some(theme.primary.base.alpha(0.8).into());
        }
        if status.expanded {
            match self.kind {
                Variant::Outline | Variant::Ghost => {
                    // Dark hover has precedence over the expanded background.
                    if !dark || !status.hovered || status.disabled {
                        paint.background = Some(theme.muted.base.into());
                    }
                    paint.text_color = theme.neutral.foreground;
                }
                Variant::Secondary => {
                    paint.background = Some(theme.secondary.base.into());
                    paint.text_color = theme.secondary.foreground;
                }
                _ => {}
            }
        }
        let mut ring = None;
        if status.focused && !status.disabled {
            let destructive = matches!(self.kind, Variant::Destructive);
            paint.border.color = if destructive {
                theme.destructive.base.alpha(0.4)
            } else {
                theme.neutral.ring
            };
            let color = if destructive {
                theme.destructive.base.alpha(if dark { 0.4 } else { 0.2 })
            } else {
                theme.neutral.ring.alpha(0.5)
            };
            ring = Some(Border {
                color: color.into(),
                width: 3.0,
                ..Border::default()
            });
        }
        if status.invalid {
            paint.border.color = theme.destructive.base.alpha(if dark { 0.5 } else { 1.0 });
            ring = Some(Border {
                color: theme
                    .destructive
                    .base
                    .alpha(if dark { 0.4 } else { 0.2 })
                    .into(),
                width: 3.0,
                ..Border::default()
            });
        }
        let mut ring_offset = 0.0;
        for tweak in &self.tweaks.0 {
            apply(
                tweak,
                &mut paint,
                &mut ring,
                &mut ring_offset,
                theme,
                status,
            );
        }
        // `disabled:opacity-50` composites the element, tweaked colors included.
        if status.disabled {
            paint.background = paint.background.map(|bg| bg.scale_alpha(0.5));
            paint.text_color = paint.text_color.alpha(paint.text_color.a * 0.5);
            paint.border.color = paint.border.color.alpha(paint.border.color.a * 0.5);
            if let Some(ring) = &mut ring {
                ring.color.a *= 0.5;
            }
        }
        Style {
            paint: paint.into(),
            ring,
            underline: status.hovered && !status.disabled && matches!(self.kind, Variant::Link),
            ring_offset,
        }
    }
}

fn apply(
    tweak: &Tweak,
    paint: &mut super::Style,
    ring: &mut Option<Border>,
    ring_offset: &mut f32,
    theme: &Theme,
    status: Status,
) {
    match tweak {
        Tweak::State(trigger, tweaks) => {
            let enabled = match trigger {
                Trigger::Hover => status.hovered && !status.disabled,
                Trigger::Active => status.pressed && !status.disabled,
                Trigger::Focus => status.focused && !status.disabled,
                Trigger::Disabled => status.disabled,
            };
            if enabled {
                for tweak in &tweaks.0 {
                    apply(tweak, paint, ring, ring_offset, theme, status);
                }
            }
        }
        Tweak::Ring {
            color,
            width,
            offset,
        } => {
            *ring = Some(Border {
                color: color.resolve(theme).into(),
                width: *width,
                ..Border::default()
            });
            *ring_offset = *offset;
        }
        _ => super::apply(tweak, paint, theme, status.stock()),
    }
}

impl<'a, F> IntoStyle<'a> for style::StyleFn<F>
where
    F: Fn(Palette, style::Status) -> style::Style + 'a,
{
    // Universal callbacks have no recoverable variant identity.
    fn uses_border_box(&self) -> bool {
        false
    }
    fn into_style(self) -> StyleFn<'a> {
        Box::new(move |theme, status| {
            let s = status.universal();
            let user = (self.inner)(theme.tinted(self.tint), s);
            let merged = super::intrinsic_default(theme, s) + user;
            let ring = merged.ring.map(|color| Border {
                color: color.into(),
                width: merged.ring_width.unwrap_or(3.0),
                ..Border::default()
            });
            let ring_offset = merged.ring_offset.unwrap_or(0.0);
            Style {
                paint: super::project_button(merged),
                ring,
                ring_offset,
                underline: false,
            }
        })
    }
}

impl<'a, V: Clone> IntoStyle<'a> for Tweaked<V>
where
    Spec: From<V>,
{
    fn uses_border_box(&self) -> bool {
        self.clone().build::<Spec>().uses_border_box()
    }
    fn into_style(self) -> StyleFn<'a> {
        self.build::<Spec>().into_style()
    }
}

impl<'a> IntoStyle<'a> for TweakSet {
    fn into_style(self) -> StyleFn<'a> {
        Spec::from(self).into_style()
    }
}

impl<'a> IntoStyle<'a> for style::Tint {
    fn uses_border_box(&self) -> bool {
        false
    }
    fn into_style(self) -> StyleFn<'a> {
        let stock: button::StyleFn<'a, Theme> = self.into();
        stock.into_style()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::tweak::{self, Ring};

    #[test]
    fn variants_keep_link_and_focus_semantics_before_erasure() {
        for theme in [Theme::light(), Theme::dark()] {
            let focus = Status {
                focused: true,
                ..Status::default()
            };
            let primary = Spec::from(super::super::Default).resolve_state(&theme, focus);
            let destructive = Spec::from(super::super::Destructive).resolve_state(&theme, focus);
            assert_eq!(primary.ring.expect("focus ring").width, 3.0);
            assert_eq!(primary.paint.border.color, theme.neutral.ring.into());
            assert_eq!(
                destructive.paint.border.color,
                theme.destructive.base.alpha(0.4).into()
            );
            let alpha = if super::super::is_dark(&theme) {
                0.4
            } else {
                0.2
            };
            assert_eq!(destructive.ring.expect("destructive ring").color.a, alpha);
            let link = Spec::from(super::super::Link);
            assert!(!link.resolve_state(&theme, Status::default()).underline);
            assert_eq!(
                link.resolve_state(
                    &theme,
                    Status {
                        hovered: true,
                        ..focus
                    }
                )
                .paint
                .text_color,
                theme.primary.base.into()
            );
            assert!(
                link.resolve_state(
                    &theme,
                    Status {
                        hovered: true,
                        ..focus
                    }
                )
                .underline
            );
            assert!(
                !link
                    .resolve_state(
                        &theme,
                        Status {
                            hovered: true,
                            disabled: true,
                            ..focus
                        }
                    )
                    .underline
            );
        }
    }

    #[test]
    fn nova_variants_paint_on_the_pixel_grid() {
        for theme in [Theme::light(), Theme::dark()] {
            for spec in [
                Spec::from(super::super::Default),
                Spec::from(super::super::Outline),
                Spec::from(super::super::Ghost),
            ] {
                assert!(spec.resolve_state(&theme, Status::default()).paint.snap);
            }
        }
    }

    #[test]
    fn hover_surface_does_not_change_when_held_and_disabled_multiplies_alpha() {
        for theme in [Theme::light(), Theme::dark()] {
            for spec in [
                Spec::from(super::super::Outline),
                Spec::from(super::super::Ghost),
                Spec::from(super::super::Secondary),
                Spec::from(super::super::Destructive),
            ] {
                let hover = Status {
                    hovered: true,
                    ..Status::default()
                };
                assert_eq!(
                    spec.resolve_state(&theme, hover).paint,
                    spec.resolve_state(
                        &theme,
                        Status {
                            pressed: true,
                            ..hover
                        }
                    )
                    .paint
                );
                let rest = spec.resolve_state(&theme, Status::default());
                let disabled = spec.resolve_state(
                    &theme,
                    Status {
                        disabled: true,
                        ..hover
                    },
                );
                assert_eq!(disabled.paint.text_color.a, rest.paint.text_color.a * 0.5);
                assert_eq!(
                    disabled.paint.border.color.a,
                    rest.paint.border.color.a * 0.5
                );
            }
        }
    }

    #[test]
    fn stock_callbacks_preserve_every_field_without_inventing_identity() {
        for theme in [Theme::light(), Theme::dark()] {
            let stock: button::StyleFn<'_, Theme> = super::super::Link.into();
            let rich: button::StyleFn<'_, Theme> = super::super::Link.into();
            let converted = rich.into_style();
            for status in [
                Status::default(),
                Status {
                    hovered: true,
                    ..Status::default()
                },
                Status {
                    focused: true,
                    ..Status::default()
                },
                Status {
                    disabled: true,
                    ..Status::default()
                },
            ] {
                let actual = converted(&theme, status);
                assert_eq!(actual.paint, stock(&theme, status.stock()));
                assert_eq!(actual.ring, None);
                assert!(!actual.underline);
            }
        }
    }

    #[test]
    fn focus_tweaks_and_universal_ring_survive_conversion() {
        let theme = Theme::light();
        let style = Spec::from(
            super::super::Outline
                + tweak::focus(Ring {
                    color: theme.primary.base.into(),
                    width: 5.0,
                    offset: 0.0,
                }),
        );
        let focus = Status {
            focused: true,
            ..Status::default()
        };
        assert_eq!(
            style
                .resolve_state(&theme, focus)
                .ring
                .expect("tweak ring")
                .width,
            5.0
        );
        let universal = style::focus(style::ring(theme.primary.base));
        assert!(universal.into_style()(&theme, focus).ring.is_some());
    }

    #[test]
    fn invalid_expanded_and_anchor_states_are_independent() {
        let theme = Theme::light();
        let invalid = Spec::from(super::super::Default).resolve_state(
            &theme,
            Status {
                invalid: true,
                ..Status::default()
            },
        );
        assert!(invalid.ring.is_some());
        assert_eq!(invalid.paint.border.color, theme.destructive.base.into());
        let expanded = Spec::from(super::super::Outline).resolve_state(
            &theme,
            Status {
                expanded: true,
                ..Status::default()
            },
        );
        assert_eq!(expanded.paint.background, Some(theme.muted.base.into()));
        let dark = Theme::dark();
        let outline = Spec::from(super::super::Outline);
        let dark_hover = Status {
            hovered: true,
            ..Status::default()
        };
        assert_eq!(
            outline.resolve_state(&dark, dark_hover).paint.background,
            outline
                .resolve_state(
                    &dark,
                    Status {
                        expanded: true,
                        ..dark_hover
                    }
                )
                .paint
                .background
        );
        let primary = Spec::from(super::super::Default);
        let hover = Status {
            hovered: true,
            ..Status::default()
        };
        assert_eq!(
            primary.resolve_state(&theme, hover).paint,
            primary.resolve_state(&theme, Status::default()).paint
        );
        assert_ne!(
            primary
                .resolve_state(
                    &theme,
                    Status {
                        anchor: true,
                        ..hover
                    }
                )
                .paint,
            primary.resolve_state(&theme, hover).paint
        );
    }
}
