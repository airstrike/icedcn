//! SidebarInset — main content area paired with a sidebar.
//!
//! Use [`main`] for content alongside a fixed or floating sidebar. [`main_styled`] adds the
//! rounded surface used with an [`inset`](super::inset) sidebar.

use crate::compose::View;
use crate::container::Container;
use iced_core::Length::Fill;
use icedcn_core::theme::container;

/// Create a sidebar inset content area (baseline).
///
/// The main content area next to the sidebar. Takes full width and
/// height (flex-1 equivalent), used as the primary content region with
/// `fixed` and `floating` sidebars.
pub fn main<'a, Message: 'a>(content: impl Into<View<'a, Message>>) -> Container<'a, Message> {
    Container::new(content).width(Fill).height(Fill)
}

/// Create a sidebar inset content area styled for the Inset sidebar variant.
///
/// Adds the inset-variant chrome (`rounded-xl border shadow-sm`) so the
/// content area reads as a card floating against the sidebar background.
/// Pair with [`super::inset`] for the matching sidebar.
pub fn main_styled<'a, Message: 'a>(
    content: impl Into<View<'a, Message>>,
) -> Container<'a, Message> {
    Container::new(content)
        .width(Fill)
        .height(Fill)
        .style(container::SidebarInset)
}
