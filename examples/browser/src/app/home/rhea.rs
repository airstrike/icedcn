//! The Rhea style the upstream showcase cards are built with: Nova's
//! components with rounder corners, filled fields and compact controls.

use iced::Vector;
use icedcn::core::{Border, Color, Radius, Shadow};
use icedcn::theme::tweak::{Alpha, Bg, Token, border, hover};
use icedcn::theme::{button, checkbox, container, radio, switch, text_input};
use icedcn::{Theme, radius};

/// `rounded-2xl`.
const ROUNDED: f32 = radius::XL2;

/// `cn-card`: `rounded-[min(var(--radius-4xl),24px)] shadow-sm ring-1
/// ring-foreground/5 dark:ring-foreground/10`.
pub fn card(theme: &Theme) -> container::Style {
    let dark = theme.neutral.is_dark;
    container::Style {
        background: Some(theme.card.base.into()),
        border: Border {
            radius: radius::XL4.min(24.).into(),
            width: 1.,
            color: theme
                .neutral
                .foreground
                .alpha(if dark { 0.1 } else { 0.05 }),
        },
        shadow: Shadow {
            color: Color::BLACK.alpha(0.1),
            offset: Vector::new(0., 1.),
            blur_radius: 3.,
        },
        ..container::Style::default()
    }
}

/// `cn-button` with the default or secondary variant.
pub fn button(variant: impl Into<button::Spec>) -> button::Spec {
    variant.into() + Radius::from(ROUNDED)
}

/// `cn-button-variant-outline`: `border-border bg-background
/// dark:bg-transparent hover:bg-muted dark:hover:bg-input/30`. On a card
/// the background reads as transparent in both modes.
pub fn outline() -> button::Spec {
    (button::Outline
        + Radius::from(ROUNDED)
        + border::Color(Token::Border)
        + Alpha(0.)
        + hover(Bg(Token::Input) + Alpha(0.3)))
    .into()
}

/// `cn-input-group` and `cn-textarea`: `bg-input/50 border-transparent`,
/// `focus-visible:border-ring ring-ring/30`.
pub fn field<'a>() -> text_input::StyleFn<'a, Theme> {
    Box::new(|theme, status| {
        let nova = text_input::default(theme, status);
        let focused = matches!(status, text_input::Status::Focused { .. });
        text_input::Style {
            background: theme.input.scale_alpha(0.5).into(),
            border: Border {
                radius: ROUNDED.into(),
                color: if focused {
                    nova.border.color
                } else {
                    Color::TRANSPARENT
                },
                ..nova.border
            },
            ring: nova.ring.map(|ring| text_input::Ring {
                color: theme.neutral.ring.alpha(0.3),
                ..ring
            }),
            ..nova
        }
    })
}

/// `cn-radio-group-item`: `bg-input/90 border-transparent
/// data-checked:bg-primary`.
pub fn radio(theme: &Theme, status: radio::Status) -> radio::Style {
    let nova = radio::default(theme, status);
    let (radio::Status::Active { is_selected } | radio::Status::Hovered { is_selected }) = status;
    radio::Style {
        background: if is_selected {
            nova.background
        } else {
            theme.input.scale_alpha(0.9).into()
        },
        border_color: iced::Color::TRANSPARENT,
        ..nova
    }
}

/// `cn-checkbox`: `bg-input/90 border-transparent rounded-[5px]`, primary
/// while checked.
pub fn checkbox<'a>() -> checkbox::StyleFn<'a, Theme> {
    Box::new(|theme, status| {
        let nova = checkbox::default(theme, status);
        let (checkbox::Status::Active { is_checked }
        | checkbox::Status::Hovered { is_checked }
        | checkbox::Status::Disabled { is_checked }) = status;
        checkbox::Style {
            background: if is_checked {
                nova.background
            } else {
                theme.input.scale_alpha(0.9).into()
            },
            border: Border {
                radius: 5.0.into(),
                color: if is_checked {
                    nova.border.color
                } else {
                    Color::TRANSPARENT
                },
                ..nova.border
            },
            ..nova
        }
    })
}

/// `cn-switch` track: 32 × 20.
pub const TRACK: (f32, f32) = (32., 20.);

/// `cn-switch`: `border-2 data-checked:border-primary
/// data-unchecked:bg-input/90 data-unchecked:border-transparent`, a 16px
/// thumb inside the 2px border.
pub fn switch<'a>() -> switch::StyleFn<'a, Theme> {
    Box::new(|theme, status| {
        let nova = switch::default(theme, status);
        let (switch::Status::Active { is_toggled }
        | switch::Status::Hovered { is_toggled }
        | switch::Status::Disabled { is_toggled }) = status;
        switch::Style {
            background: if is_toggled {
                nova.background
            } else {
                theme.input.scale_alpha(0.9).into()
            },
            background_border_width: 2.,
            background_border_color: if is_toggled {
                theme.primary.base
            } else {
                Color::TRANSPARENT
            },
            padding_ratio: 2. / TRACK.1,
            ..nova
        }
    })
}
