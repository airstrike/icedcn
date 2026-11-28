//! SidebarMenu and SidebarMenuItem

use crate::column::Column;
use crate::compose::View;
use crate::row::Row;
use iced_core::Length::Fill;

/// Create a sidebar menu.
///
/// A full-width column with adjacent rows, serving as a list
/// container for menu items.
pub fn menu<'a, Message: 'a>(
    children: impl IntoIterator<Item = View<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children).width(Fill)
}

/// Create a sidebar menu item.
///
/// A row wrapper with full width, containing the item's content.
pub fn menu_item<'a, Message: 'a>(
    children: impl IntoIterator<Item = View<'a, Message>>,
) -> Row<'a, Message> {
    Row::with_children(children).width(Fill)
}
