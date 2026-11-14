//! Corner badge content retained by its Avatar until composition.

use crate::compose::Slot;

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
