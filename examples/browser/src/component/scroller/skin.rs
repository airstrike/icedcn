use iced::Vector;
use iced::widget::button::{Status, StyleFn};
use icedcn::Theme;
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::theme::{button, popover};

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

/// `cn-hover-card-content` with the outline's `rounded-2xl p-1`: Rhea's
/// `ring-foreground/5 dark:ring-foreground/10` and `shadow-lg`.
pub(super) fn outline(theme: &Theme) -> Style {
    let mut style = popover::content(theme);
    style.border.radius = 18.0.into();
    if !theme.neutral.is_dark {
        style.border.color = theme.neutral.foreground.alpha(0.05);
    }
    style.shadow.offset = Vector::new(0., 10.);
    style.shadow.blur_radius = 15.;
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

pub(super) fn entry() -> StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let mut style = button::ghost(theme, status);
        if matches!(status, Status::Hovered | Status::Pressed) {
            style.background = Some(theme.accent.base.into());
            style.text_color = theme.accent.foreground;
        }
        style.border.radius = 14.0.into();
        style.into()
    })
}

pub(super) fn trigger() -> StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let mut style = button::ghost(theme, status);
        style.background = None;
        style.into()
    })
}
