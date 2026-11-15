//! SidebarContent: scrollable, flex-1, overflow-auto

use crate::column::Column;
use crate::compose::Slot;
use iced_core::Length::Fill;

/// Create scrollable sidebar content.
///
/// A column of adjacent groups that fills available height and width.
/// The caller can wrap this in an iced scrollable if needed.
pub fn content<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children).height(Fill).width(Fill)
}
