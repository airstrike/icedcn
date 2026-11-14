//! A preview panel that appears when the pointer rests on its trigger.
//!
//! A card shown next to its trigger while the pointer rests on it, over
//! iced's [`anchored`](iced_widget::anchored) overlay in hover mode. The
//! pointer can cross the gap into the card and stays welcome there; the
//! card closes `close_delay` after the pointer has left both.
//!
//! ```ignore
//! use icedcn::widget::{button, column, hover_card, text};
//!
//! hover_card(
//!     button("Hover Here").style(button::Link),
//!     column![text("iced").semibold(), text("A GUI library for Rust.")],
//! )
//! .open_delay(Duration::from_millis(10))
//! .close_delay(Duration::from_millis(100))
//! ```

use std::time::Duration;

use iced_core::{Element, Pixels};
use iced_widget::{Renderer, Widget, anchored};
use icedcn_core::theme::container::Style;
use icedcn_core::theme::popover;
use icedcn_core::{Padding, Theme};

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;

pub use iced_widget::anchored::{Align, Side};

/// A card opened by hovering its trigger.
pub struct HoverCard<'a, Message> {
    trigger: Slot<'a, Message>,
    content: Slot<'a, Message>,
    position: anchored::Position,
    open_delay: Duration,
    close_delay: Duration,
    width: f32,
    padding: Padding,
    style: fn(&Theme) -> Style,
    open: Option<bool>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

/// Create a [`HoverCard`] showing `content` while `trigger` is hovered.
pub fn hover_card<'a, Message>(
    trigger: impl Into<Slot<'a, Message>>,
    content: impl Into<Slot<'a, Message>>,
) -> HoverCard<'a, Message> {
    HoverCard {
        trigger: trigger.into(),
        content: content.into(),
        position: anchored::Position::new(Side::Bottom).offset(4),
        open_delay: Duration::from_millis(700),
        close_delay: Duration::from_millis(300),
        // `w-64`.
        width: 256.0,
        padding: 10.into(),
        style: popover::content,
        open: None,
        on_open_change: None,
    }
}

impl<'a, Message> HoverCard<'a, Message> {
    /// Space between the panel edge and its content.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set the floating panel's appearance.
    pub fn style(mut self, style: fn(&Theme) -> Style) -> Self {
        self.style = style;
        self
    }

    /// Set how long the pointer rests on the trigger before the card opens
    /// (700ms by default).
    pub fn open_delay(mut self, delay: Duration) -> Self {
        self.open_delay = delay;
        self
    }

    /// Set how long the card stays after the pointer leaves (300ms by
    /// default).
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.close_delay = delay;
        self
    }

    /// Set the side of the trigger the card prefers (`Bottom` by default).
    pub fn side(mut self, side: Side) -> Self {
        self.position = self.position.side(side);
        self
    }

    /// Set how the card lines up with its trigger (`Center` by default).
    pub fn align(mut self, align: Align) -> Self {
        self.position = self.position.align(align);
        self
    }

    /// Set the gap between the trigger and the card (4px by default).
    pub fn side_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.position = self.position.offset(offset);
        self
    }

    /// Set the card's width (`w-64`, 256px, by default).
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into().0;
        self
    }

    /// Control the card: it shows exactly when `open` is, and hovering only
    /// publishes [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Set the message published when hovering opens or closes the card.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for HoverCard<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.trigger.inherit(from);
        self.content.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let HoverCard {
            trigger,
            content: body,
            position,
            open_delay,
            close_delay,
            width,
            padding,
            style,
            open,
            on_open_change,
        } = *self;

        // `p-2.5 rounded-lg bg-popover shadow-md ring-1`.
        let panel = Container::new(body)
            .padding(padding)
            .width(width)
            .style(style);

        let mut card = anchored(trigger.compose(), Slot::from(panel).compose())
            .position(position)
            .hover(open_delay, close_delay);

        if let Some(open) = open {
            card = card.open(open);
        }

        if let Some(on_change) = on_open_change {
            card = card.on_toggle(on_change);
        }

        card.boxed()
    }
}

impl<'a, Message: 'a> From<HoverCard<'a, Message>> for Slot<'a, Message> {
    fn from(card: HoverCard<'a, Message>) -> Self {
        Slot::new(card)
    }
}

impl<'a, Message: 'a> From<HoverCard<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(card: HoverCard<'a, Message>) -> Self {
        Box::new(card).compose()
    }
}
