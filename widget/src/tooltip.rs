//! A short hint anchored to a widget.
//!
//! A `text-xs` hint on a `bg-foreground` panel with an arrow pointing at its
//! trigger, shown on pointer hover or keyboard focus, on the trigger's
//! [`Side`] (top by default). Built on iced's
//! [`anchored`](iced_widget::anchored) overlay in hover mode, so it flips
//! and shifts to stay in the window, and `Escape` hides it.
//!
//! # Example
//!
//! ```ignore
//! use icedcn::widget::{button, tooltip};
//!
//! tooltip(button("Hover").style(button::Outline), "Add to library")
//!
//! tooltip(button("Left").style(button::Outline), "Add to library")
//!     .side(tooltip::Side::Left)
//! ```

pub(crate) mod interaction;
use interaction::{Interaction, Mode};
use std::time::Duration;

use iced_core::Pixels;
use iced_widget::{Renderer, Widget, anchored};
use icedcn_core::Padding;
use icedcn_core::Theme;
use icedcn_core::theme::tooltip::content;

use crate::Size;
use crate::compose::{Compose, Inherited, View};

pub use iced_widget::anchored::{Align, Side};

/// A trigger that shows a hint while hovered.
pub struct Tooltip<'a, Message> {
    trigger: View<'a, Message>,
    content: View<'a, Message>,
    position: anchored::Position,
    delay: Duration,
    padding: Padding,
    default_open: bool,
    open: Option<bool>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

/// Create a [`Tooltip`] showing `content` while `trigger` is hovered.
pub fn tooltip<'a, Message: 'a>(
    trigger: impl Into<View<'a, Message>>,
    content: impl Into<View<'a, Message>>,
) -> Tooltip<'a, Message> {
    Tooltip {
        trigger: trigger.into(),
        content: content.into(),
        // Include the arrow height in the distance between the trigger and panel.
        position: anchored::Position::new(Side::Top).offset(10),
        delay: Duration::ZERO,
        padding: [6, 12].into(),
        default_open: false,
        open: None,
        on_open_change: None,
    }
}

impl<'a, Message: 'a> Tooltip<'a, Message> {
    /// Set the side of the trigger the tooltip appears on.
    pub fn side(mut self, side: Side) -> Self {
        self.position = self.position.side(side);
        self
    }

    /// Set how the tooltip lines up with its trigger (`Center` by default).
    pub fn align(mut self, align: Align) -> Self {
        self.position = self.position.align(align);
        self
    }

    /// Set the gap to the trigger, the arrow included (10px by default).
    pub fn side_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.position = self.position.offset(offset);
        self
    }

    /// Set how long the pointer rests on the trigger before the tooltip
    /// shows (`delayDuration`, none by default).
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set the space around the hint. The default is 6px vertically and 12px horizontally.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Whether an uncontrolled tooltip starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Control the tooltip: it shows exactly when `open` is, and hovering
    /// only publishes [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Set the message published when hovering shows or hides the tooltip.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Tooltip<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.trigger.inherit(from);
    }

    fn compose<'b>(mut self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        self.content
            .inherit(&Inherited::default().with_size(Size::Xs));

        let panel = crate::container(self.content)
            .padding(self.padding)
            .max_width(320)
            .style(|theme: &Theme| content(theme));

        Interaction::new(
            self.trigger.compose(),
            View::from(panel).compose(),
            self.position,
            (self.delay, Duration::ZERO),
            Mode::Hint,
            self.open,
            self.on_open_change,
        )
        .default_open(self.default_open)
        .boxed()
    }
}

impl<'a, Message: 'a> From<Tooltip<'a, Message>> for View<'a, Message> {
    fn from(tooltip: Tooltip<'a, Message>) -> Self {
        View::new(tooltip)
    }
}

impl<'a, Message: 'a> From<Tooltip<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(tooltip: Tooltip<'a, Message>) -> Self {
        Box::new(tooltip).compose()
    }
}
