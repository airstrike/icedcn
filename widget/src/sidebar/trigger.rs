//! SidebarTrigger — toggle button for collapsing/expanding the sidebar
//!
//! A 28-pixel square button that toggles the nearest enclosing [`provider`](super::provider).
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::sidebar;
//!
//! sidebar::provider(|s| {
//!     row![s.sidebar(menu()), sidebar::main(column![sidebar::trigger(), body()])].into()
//! })
//! ```

use crate::Size;
use crate::button::{self, button};
use crate::compose::{Compose, Inherited, Slot};
use crate::icon::Icon;
use crate::marker::{self, Press};
use iced_core::Element;
use iced_widget::Renderer;
use iced_widget::Widget;
use icedcn_core::Theme;

/// The signal a trigger records when pressed.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Toggle;

/// Create a sidebar trigger button (ghost, 28x28) that toggles the enclosing
/// [`provider`](super::provider).
pub fn trigger<'a, Message: Clone + 'a>() -> Slot<'a, Message> {
    Slot::new(Trigger {
        button: button(Icon::PanelLeft.boxed())
            .size(Size::Sm)
            .square()
            .style(button::Ghost),
    })
}

/// The trigger button, marked to record [`Toggle`].
struct Trigger<'a, Message> {
    button: crate::Button<'a, Message>,
}

impl<'a, Message: Clone + 'a> Compose<Message> for Trigger<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        Compose::inherit(
            &mut self.button,
            &Inherited::default().with_disabled(from.is_disabled()),
        );
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        marker::marker(Slot::from(self.button).compose(), Toggle, Press::Captured).into()
    }
}
