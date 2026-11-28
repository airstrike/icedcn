//! A menu opened from a button or other trigger.
//!
//! A [`Popover`] holding a menu: its panel is as wide as its trigger (at
//! least 128px) with `p-1`, and it is modal, so the page ignores the
//! pointer while it is open and a press anywhere outside closes it. Build
//! the content from [`menu`](crate::menu) parts; selecting an item closes
//! the menu. A focused trigger also opens with ArrowDown and focuses the first item. Pointer opening waits until an item is hovered or a key is pressed.
//!
//! ```ignore
//! use icedcn::widget::{button, column, dropdown_menu, menu};
//!
//! dropdown_menu(
//!     |m| m.trigger(button("Open").style(button::Outline)),
//!     |_| {
//!         column![
//!             menu::label("My Account"),
//!             menu::item("Profile").shortcut("⇧⌘P"),
//!             menu::separator(),
//!             menu::item("Log out").on_select(Message::LogOut),
//!         ]
//!         .into()
//!     },
//! )
//! .align(dropdown_menu::Align::Start)
//! .width(160)
//! ```

use iced_core::{Element, Pixels};
use iced_widget::Renderer;
use icedcn_core::Theme;

use crate::compose::{Compose, Inherited, View};
use crate::popover::{self, Popover};

pub use crate::popover::{Align, Handle, Side};

#[cfg(test)]
mod tests;

/// A menu opened from its trigger.
pub struct DropdownMenu<'a, Message> {
    popover: Popover<'a, Message>,
}

/// Create a closed, uncontrolled [`DropdownMenu`]: `trigger` builds what
/// sits in place, `content` the menu shown while open.
pub fn dropdown_menu<'a, Message>(
    trigger: impl Fn(&Handle) -> View<'a, Message> + 'a,
    content: impl Fn(&Handle) -> View<'a, Message> + 'a,
) -> DropdownMenu<'a, Message> {
    DropdownMenu {
        // Match the trigger width with a 128-pixel minimum and four pixels of padding.
        popover: popover::popover(trigger, content)
            .align(Align::Start)
            .trigger_width(128.0)
            .padding(4)
            .modal(true)
            .open_on_arrow_down(),
    }
}

impl<'a, Message> DropdownMenu<'a, Message> {
    /// Reveal the existing selection when this menu serves as a calendar caption.
    pub(crate) fn initial_choice(mut self, index: usize, height: f32) -> Self {
        self.popover = self.popover.initial_choice(index, height);
        self
    }

    /// Control the menu: it is open exactly when `open` is.
    pub fn open(mut self, open: bool) -> Self {
        self.popover = self.popover.open(open);
        self
    }

    /// Whether an uncontrolled menu starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.popover = self.popover.default_open(open);
        self
    }

    /// Set the message published with the new open state.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.popover = self.popover.on_open_change(on_change);
        self
    }

    /// Set the side of the trigger the menu prefers (`Bottom` by default).
    pub fn side(mut self, side: Side) -> Self {
        self.popover = self.popover.side(side);
        self
    }

    /// Set how the menu lines up with its trigger (`Center` by default).
    pub fn align(mut self, align: Align) -> Self {
        self.popover = self.popover.align(align);
        self
    }

    /// Set the gap between the trigger and the menu (4px by default).
    pub fn side_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.popover = self.popover.side_offset(offset);
        self
    }

    /// Set an explicit menu width. By default the menu matches the trigger width.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.popover = self.popover.width(width);
        self
    }
}

impl<'a, Message: 'static> Compose<Message> for DropdownMenu<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        Compose::inherit(&mut self.popover, from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        Box::new(self.popover).compose()
    }
}

impl<'a, Message: 'static> From<DropdownMenu<'a, Message>> for View<'a, Message> {
    fn from(menu: DropdownMenu<'a, Message>) -> Self {
        View::new(menu)
    }
}

impl<'a, Message: 'static> From<DropdownMenu<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(menu: DropdownMenu<'a, Message>) -> Self {
        menu.popover.into()
    }
}
