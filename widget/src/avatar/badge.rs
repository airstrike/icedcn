//! Corner badge content retained by its Avatar until composition.

use crate::compose::Slot;
use icedcn_core::theme::{avatar::badge_style, container::Style};
use icedcn_core::{Color, Theme};

use super::Avatar;

/// Add a badge at the avatar's bottom right without changing its footprint.
///
/// The returned Avatar can still be sized, styled or placed in a group.
/// Use `.badge_style(..)` to customize the badge surface. Compose-aware
/// content inherits an 8px size; explicitly size stock iced icons to 8px.
/// Small avatars hide the content but keep their presence dot.
pub fn badge<'a, Message: 'a>(
    mut avatar: Avatar<'a, Message>,
    content: impl Into<Slot<'a, Message>>,
) -> Avatar<'a, Message> {
    avatar.badge = Some(content.into());
    avatar
}

/// Empty badge content for a presence dot.
pub fn dot<'a, Message: 'a>() -> Slot<'a, Message> {
    Slot::from("")
}

/// Green presence indicator, with a darker fill in dark themes.
pub fn presence(theme: &Theme) -> Style {
    let mut style = badge_style(theme);
    style.background = Some(
        Color::from_srgb(if theme.neutral.is_dark {
            iced_core::Color::from_rgb8(1, 102, 48)
        } else {
            iced_core::Color::from_rgb8(0, 166, 62)
        })
        .into(),
    );
    style.border.width = 0.0;
    style
}
