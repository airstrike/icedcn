//! A full-height, 16-pixel-wide rail for toggling the sidebar. On hover it shows a horizontal
//! resize cursor and the sidebar border color.
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::sidebar;
//!
//! sidebar::rail(Message::ToggleSidebar)
//! ```

use iced_core::Length::Fill;
use iced_core::mouse::Interaction;
use iced_widget::Renderer;
use iced_widget::button::Status;
use iced_widget::space;

use super::trigger::Toggle;
use crate::button::widget;
use crate::compose::{Compose, Slot};
use crate::marker::{self, Press};
use icedcn_core::Theme;
use icedcn_core::theme::button;

/// Width of the rail hit target in logical pixels.
pub const RAIL_WIDTH: f32 = 16.0;

/// A thin click-target rail that toggles the sidebar.
pub struct SidebarRail<Message> {
    on_press: Option<Message>,
}

/// Create a sidebar rail.
///
/// Emits `on_press` when clicked. Renders as a 16px-wide button spanning
/// the full sidebar height with a horizontal-resize cursor.
pub fn rail<Message: Clone>(on_press: Message) -> SidebarRail<Message> {
    SidebarRail {
        on_press: Some(on_press),
    }
}

/// A resize rail linked to its enclosing sidebar provider.
pub fn rail_trigger<'a, Message: Clone + 'a>() -> Slot<'a, Message> {
    let rail = SidebarRail { on_press: None };
    let element: iced_core::Element<'a, Message, Theme, Renderer> = rail.into();
    {
        let element: iced_core::Element<'a, Message, Theme, Renderer> =
            marker::marker(element, Toggle, Press::Captured).into();
        element.into()
    }
}

// -- From impls ---------------------------------------------------------------

impl<'a, Message: Clone + 'a> From<SidebarRail<Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(rail: SidebarRail<Message>) -> Self {
        // Show the sidebar border color when the pointer is over the rail.
        let style: widget::StyleFn<'a, Theme> = Box::new(|t, s: widget::Status| {
            let status: Status = s.into();
            let mut core = button::ghost(t, status);
            if matches!(status, Status::Hovered | Status::Pressed) {
                core.background = Some(t.sidebar.border.into());
            }
            let iced: iced_widget::button::Style = core.into();
            iced.into()
        });

        let raw_btn = widget::Button::new(space::horizontal())
            .width(RAIL_WIDTH)
            .height(Fill)
            .padding(0)
            .translate(false)
            .interaction(Interaction::ResizingHorizontally)
            .style(style)
            .on_press_maybe(rail.on_press);

        raw_btn.into()
    }
}

impl<'a, Message: Clone + 'a> From<SidebarRail<Message>> for crate::Slot<'a, Message> {
    fn from(widget: SidebarRail<Message>) -> Self {
        crate::Slot::new(widget)
    }
}

impl<Message: Clone> Compose<Message> for SidebarRail<Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        (*self).into()
    }
}
