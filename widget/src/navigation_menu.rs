//! Navigation menus combine expandable groups of links with standalone links.
use crate::menubar::{self, Entry, Menubar};
use std::borrow::Cow;

pub use crate::menubar::menu;

/// A standalone top-level link that publishes its destination when activated.
pub fn link<'a, Message>(label: impl Into<Cow<'a, str>>, message: Message) -> Entry<'a, Message> {
    menubar::link(label, message)
}

/// A horizontal navigation menu. Groups open on pointer hover or keyboard activation.
pub fn navigation_menu<'a, Message>(
    entries: impl IntoIterator<Item = Entry<'a, Message>>,
) -> Menubar<'a, Message> {
    menubar::menubar(entries).navigation()
}
