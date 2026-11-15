//! SidebarMenuSub, SidebarMenuSubItem, SidebarMenuSubButton

use crate::column::Column;
use crate::compose::Slot;
use crate::container::Container;
use crate::row::Row;
use iced_core::Length::Fill;
use iced_core::Padding;
use iced_widget::Widget;
use icedcn_core::Theme;

/// Create a nested sidebar list with indentation and a left border.
pub fn submenu<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Row<'a, Message> {
    // Left border indicator
    let border_line: iced_core::Element<'a, Message, Theme, iced_widget::Renderer> =
        iced_widget::container(iced_widget::space::horizontal())
            .width(1.0)
            .height(Fill)
            .style(|theme: &Theme| iced_widget::container::Style {
                background: Some(theme.sidebar.border.into()),
                ..Default::default()
            })
            .boxed();

    // Items column with padding
    let items = Column::with_children(children)
        .spacing(4)
        .padding(Padding {
            top: 2.0,
            bottom: 2.0,
            left: 10.0,
            right: 10.0,
        })
        .width(Fill);

    Row::with_children([Slot::from(border_line), items.into()])
        .padding([0, 14])
        .width(Fill)
}

/// Create a sidebar submenu item.
///
/// A simple row wrapper for submenu content.
pub fn submenu_item<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Row<'a, Message> {
    Row::with_children(children).width(Fill)
}

/// A 28px-high submenu slot. Pass an interactive menu button as its content
/// when a destination should publish a navigation message.
pub fn submenu_button<'a, Message: 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(content).width(Fill).height(28)
}
