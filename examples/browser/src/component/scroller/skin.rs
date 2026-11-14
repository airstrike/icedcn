use iced::Vector;
use iced::widget::button::{Status, StyleFn};
use icedxdy::core::{Border, Shadow};
use icedxdy::theme::container::Style;
use icedxdy::theme::{button, popover};
use icedxdy::{Color, Theme};

pub(super) fn card(theme: &Theme) -> Style {
    let alpha = if theme.neutral.is_dark { 0.1 } else { 0.05 };
    Style {
        background: Some(theme.card.base.into()),
        border: Border {
            radius: 24.0.into(),
            width: 1.0,
            color: theme.neutral.foreground.alpha(alpha),
        },
        shadow: Shadow {
            color: Color::BLACK.alpha(0.10),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 3.0,
        },
        ..Style::default()
    }
}

pub(super) fn tile(theme: &Theme) -> Style {
    Style {
        background: Some(theme.muted.base.into()),
        border: Border {
            radius: 14.0.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

pub(super) fn outline(theme: &Theme) -> Style {
    let mut style = popover::content(theme);
    style.border.radius = 18.0.into();
    style
}

/// Rhea's outline has a transparent dark surface and halves the whole
/// control's opacity when disabled, including its already translucent border.
pub(super) fn outlined() -> StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let mut style = button::outline(theme, status);
        if theme.neutral.is_dark {
            style.background = match status {
                Status::Hovered | Status::Pressed => {
                    Some(theme.input.alpha(theme.input.a * 0.3).into())
                }
                _ => None,
            };
            let border = theme.neutral.border;
            style.border.color = if status == Status::Disabled {
                border.alpha(border.a * 0.5)
            } else {
                border
            };
        }
        style.into()
    })
}
