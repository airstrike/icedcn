//! Toast colors, rounded border, and shadow. Action buttons invert the toast’s foreground and
//! background colors.

use iced_core::Vector;

use super::Theme;
use super::button;
use super::container::Style;
use crate::{Border, Color, Shadow, radius};

/// A toast card.
pub fn card(theme: &Theme) -> Style {
    Style {
        background: Some(theme.popover.base.into()),
        text_color: Some(theme.popover.foreground),
        border: Border {
            radius: radius::LG.into(),
            width: 1.0,
            color: theme.neutral.border,
        },
        shadow: Shadow {
            color: Color::BLACK.alpha(0.1),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        ..Style::default()
    }
}

/// A toast's action button: the toast's text color as its fill.
pub fn action(theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered | button::Status::Pressed => theme.popover.foreground.alpha(0.9),
        button::Status::Active | button::Status::Disabled => theme.popover.foreground,
    };

    button::Style {
        background: Some(background.into()),
        text_color: theme.popover.base,
        border: Border {
            radius: radius::XS.into(),
            ..Border::default()
        },
        ..button::Style::default()
    }
}

/// The supporting line in a notification.
pub fn description(theme: &Theme) -> iced_widget::text::Style {
    let shade = if theme.neutral.base.l < 0.5 { 232 } else { 63 };
    iced_widget::text::Style {
        color: Some(iced_core::Color::from_rgb8(shade, shade, shade)),
        ..Default::default()
    }
}
