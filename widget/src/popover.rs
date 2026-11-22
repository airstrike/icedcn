//! A panel anchored to its trigger. Two closures receive a [`Handle`]. Use the first to place a
//! trigger and the second to build the panel. The handle can turn a child into a toggle or
//! close control.
//!
//! ```ignore
//! use icedcn::widget::{button, popover};
//!
//! popover(
//!     |p| p.trigger(button("Open popover").style(button::Outline)),
//!     |_| popover::header("Dimensions", "Set the dimensions for the layer."),
//! )
//! .align(popover::Align::Start)
//! // Controlled: .open(self.is_open).on_open_change(Message::OpenChanged)
//! ```
//!
//! The panel sits 4px below its trigger, centered, flips above it when the
//! window has no room below and slides along it to stay inside. A press
//! outside the panel (the page still receives it) and `Escape` close it;
//! pressing the trigger again does too. The panel is built only while open,
//! so its widgets start with fresh state each time.

use iced_core::Length::Fill;
use iced_core::keyboard::key::Named;
use iced_core::{Element, Length, Padding, Pixels};
use iced_widget::anchored;
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Space, Widget};
use icedcn_core::Theme;
use icedcn_core::theme;
use icedcn_core::theme::popover::content;
use std::time::Duration;

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};
use crate::menu::list::{Initial, List};
use crate::{column, text};
use iced_core::widget::Id;

mod focus;

pub use iced_widget::anchored::{Align, Side};

/// A closure that builds part of a [`Popover`].
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// A panel anchored to its trigger, opened and closed by it.
pub struct Popover<'a, Message> {
    trigger: Body<'a, Message>,
    content: Body<'a, Message>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    position: anchored::Position,
    width: Option<f32>,
    min_width: f32,
    padding: Padding,
    modal: bool,
    initial_choice: Option<(usize, f32)>,
    focus_content: Option<bool>,
    open_on_arrow_down: bool,
    hover: Option<(Duration, Duration)>,
    inherited: Inherited,
}

/// What a [`Popover`]'s closures receive: whether it is open, and the
/// wrappers for its triggers and close buttons.
#[derive(Debug, Clone, Copy)]
pub struct Handle {
    is_open: bool,
    open_on_arrow_down: bool,
}

/// What a popover's markers signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    /// A trigger was pressed.
    Toggle,
    /// A close button (or a menu item) was pressed.
    Close,
}

/// The popover's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A trigger was pressed.
    Toggle,
    /// A keyboard trigger was activated.
    KeyboardToggle,
    /// The popover asked to open or close.
    Set(bool),
    /// The content published an app message.
    App(Message),
    /// `Escape` reached the open panel unclaimed, so it closes by keyboard.
    Escaped,
    /// The trigger took focus back after a keyboard dismissal.
    Refocused,
}

/// The open state of an uncontrolled popover.
#[derive(Debug, Default)]
pub struct State {
    open: Option<bool>,
    keyboard: bool,
    /// A keyboard dismissal hands focus back to the trigger.
    refocus: bool,
}

/// Create a closed, uncontrolled [`Popover`]: `trigger` builds what sits in
/// place, `content` the panel shown while open.
pub fn popover<'a, Message>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
) -> Popover<'a, Message> {
    Popover {
        trigger: Box::new(trigger),
        content: Box::new(content),
        open: None,
        default_open: false,
        on_open_change: None,
        position: anchored::Position::new(Side::Bottom).offset(4),
        // `w-72`.
        width: Some(288.0),
        min_width: 0.0,
        // `p-2.5`.
        padding: Padding::new(10.0),
        modal: false,
        initial_choice: None,
        focus_content: Some(false),
        open_on_arrow_down: false,
        hover: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Popover<'a, Message> {
    pub(crate) fn initial_choice(mut self, index: usize, height: f32) -> Self {
        self.initial_choice = Some((index, height));
        self
    }

    /// Control the popover: it is open exactly when `open` is, and its
    /// triggers and dismissals only publish
    /// [`on_open_change`](Self::on_open_change).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Whether an uncontrolled popover starts open.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Set the message published with the new open state when the popover
    /// is opened or dismissed.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }

    /// Set the side of the trigger the panel prefers (`Bottom` by default).
    pub fn side(mut self, side: Side) -> Self {
        self.position = self.position.side(side);
        self
    }

    /// Set how the panel lines up with its trigger (`Center` by default).
    pub fn align(mut self, align: Align) -> Self {
        self.position = self.position.align(align);
        self
    }

    /// Set the gap between the trigger and panel, four pixels by default.
    pub fn side_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.position = self.position.offset(offset);
        self
    }

    /// Move a start- or end-aligned panel inward along the trigger
    /// (`alignOffset`).
    pub fn align_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.position = self.position.align_offset(offset);
        self
    }

    /// Set the panel's width (`w-72`, 288px, by default).
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into().0);
        self
    }

    /// Match the trigger width while keeping the panel at least `min_width` wide.
    pub(crate) fn trigger_width(mut self, min_width: f32) -> Self {
        self.width = None;
        self.min_width = min_width;
        self
    }

    /// Set the panel's padding.
    pub(crate) fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Make the panel modal: the page ignores the pointer while it is open
    /// (a dropdown menu).
    pub(crate) fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Focus the first control when mounting a keyboard-navigable popup.
    pub(crate) fn focus_content(mut self, keyboard: bool) -> Self {
        self.focus_content = Some(keyboard);
        self
    }

    /// Dropdowns opt into keyboard opening; ordinary popovers keep their keys.
    pub(crate) fn open_on_arrow_down(mut self) -> Self {
        self.open_on_arrow_down = true;
        self
    }

    pub(crate) fn hover(mut self, open_delay: Duration, close_delay: Duration) -> Self {
        self.hover = Some((open_delay, close_delay));
        self.focus_content = None;
        self
    }

    fn is_open(&self, state: &State) -> bool {
        self.open.or(state.open).unwrap_or(self.default_open)
    }
}

impl Handle {
    pub(crate) fn new(is_open: bool) -> Self {
        Self {
            is_open,
            open_on_arrow_down: false,
        }
    }

    /// Whether the popover is open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Pressing the child toggles the popover and runs the child’s own handlers.
    pub fn trigger<'a, Message: 'a>(
        &self,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        self.trigger_with_control(child, false)
    }

    pub(crate) fn trigger_with_control<'a, Message: 'a>(
        &self,
        child: impl Into<Slot<'a, Message>>,
        form_control: bool,
    ) -> Slot<'a, Message> {
        Slot::new(Marked {
            child: child.into(),
            form_control,
            signal: Signal::Toggle,
            press: Press::Captured,
            open_on_arrow_down: self.open_on_arrow_down && !self.is_open,
        })
    }

    /// Pressing the child closes the popover and runs the child’s own handlers.
    pub fn close<'a, Message: 'a>(&self, child: impl Into<Slot<'a, Message>>) -> Slot<'a, Message> {
        Slot::new(Marked {
            child: child.into(),
            signal: Signal::Close,
            form_control: false,
            press: Press::Captured,
            open_on_arrow_down: false,
        })
    }
}

/// A `PopoverHeader`: a `PopoverTitle` over a `PopoverDescription`.
pub fn header<'a, Message: 'a>(title: &'a str, description: &'a str) -> Slot<'a, Message> {
    column![
        text::Text::new(title)
            .size(text::size::SM)
            .medium()
            .style(theme::text::Foreground),
        text::muted(description),
    ]
    .spacing(2)
    .into()
}

/// A child that records a [`Signal`] when pressed.
pub(crate) struct Marked<'a, Message> {
    pub(crate) child: Slot<'a, Message>,
    pub(crate) signal: Signal,
    pub(crate) press: Press,
    form_control: bool,
    open_on_arrow_down: bool,
}

impl<'a, Message: 'a> Compose<Message> for Marked<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.child.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let mut marker = marker::marker(self.child.compose(), self.signal, self.press);
        if self.form_control {
            marker = marker.control(false);
        }
        if self.open_on_arrow_down {
            marker.on_key(Named::ArrowDown, Signal::Toggle).into()
        } else {
            marker.into()
        }
    }
}

fn on_signal<Message>(signal: Signal) -> Event<Message> {
    match signal {
        Signal::Toggle => Event::Toggle,
        Signal::Close => Event::Set(false),
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Popover<'a, Message> {
    type State = State;
    type Event = Event<Message>;

    fn diff(&mut self, state: &mut State) {
        if self.open.is_none() && state.open.is_none() {
            state.open = Some(self.default_open);
        }
    }

    fn update(
        &self,
        state: &mut State,
        event: Event<Message>,
        _renderer: &Renderer,
    ) -> Option<Message> {
        let next = match event {
            Event::App(message) => return Some(message),
            Event::Escaped => {
                state.refocus = true;
                return None;
            }
            Event::Refocused => {
                state.refocus = false;
                return None;
            }
            Event::Toggle | Event::KeyboardToggle => {
                state.keyboard = matches!(event, Event::KeyboardToggle);
                !self.is_open(state)
            }
            Event::Set(open) => open,
        };

        if next == self.is_open(state) {
            return None;
        }
        if next {
            state.refocus = false;
        }

        if self.open.is_none() {
            state.open = Some(next);
        }

        self.on_open_change
            .as_ref()
            .map(|on_change| on_change(next))
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let handle = Handle {
            is_open: self.is_open(state),
            open_on_arrow_down: self.open_on_arrow_down,
        };

        let mut trigger = (self.trigger)(&handle);
        trigger.inherit(&self.inherited);
        let trigger = marker::owner_with_event(
            trigger.compose().map(Event::App).boxed(),
            |signal, event| {
                if signal == Signal::Toggle && matches!(event, iced_core::Event::Keyboard(_)) {
                    Event::KeyboardToggle
                } else {
                    on_signal(signal)
                }
            },
        );
        // Always wrapped, so the trigger keeps its state across a refocus.
        let trigger = focus::Refocus {
            content: Element::from(trigger),
            pending: state.refocus && !handle.is_open,
            on_focus: || Event::Refocused,
        }
        .boxed();

        let panel: Element<'a, Event<Message>, Theme, Renderer> = if handle.is_open {
            let mut body = (self.content)(&handle);
            let inherited = self
                .inherited
                .without_join()
                .or(Inherited::default().with_size(crate::Size::Default));
            body.inherit(&inherited);

            let scroll = Id::unique();
            if self.modal {
                let mut scroller = crate::scrollable(body)
                    .id(scroll.clone())
                    .width(Fill)
                    .class(theme::scrollable::Hidden);
                if let Some((_, height)) = self.initial_choice {
                    scroller = scroller.height(height);
                }
                body = scroller.into();
            }
            let panel = Container::new(body)
                .padding(self.padding)
                .width(match self.width {
                    Some(width) => Length::Fixed(width),
                    None => Fill,
                })
                .style(content);

            let body = marker::owner(
                Slot::from(panel).compose().map(Event::App).boxed(),
                on_signal,
            )
            .boxed();
            if self.modal {
                List {
                    content: body,
                    horizontal: Some(false),
                    autofocus: Initial::Dropdown {
                        keyboard: state.keyboard,
                        selected: self.initial_choice.map(|(index, _)| index),
                        scroll,
                    },
                    on_move: None,
                }
                .boxed()
            } else if let Some(keyboard) = self.focus_content {
                List {
                    content: body,
                    horizontal: None,
                    autofocus: Initial::Content {
                        keyboard: state.keyboard || keyboard,
                    },
                    on_move: None,
                }
                .boxed()
            } else {
                body
            }
        } else {
            Space::new().boxed()
        };

        let panel = if handle.is_open {
            focus::Escape {
                content: panel,
                on_escape: || Event::Escaped,
            }
            .boxed()
        } else {
            panel
        };

        let mut anchor = anchored(trigger, panel)
            .position(self.position)
            .width(if self.width.is_some() {
                anchored::Width::Content
            } else {
                anchored::Width::AtLeastAnchor
            })
            .min_width(self.min_width)
            .open(handle.is_open)
            .modal(self.modal)
            .on_toggle(Event::Set);
        if let Some((open, close)) = self.hover {
            anchor = anchor.hover(open, close);
        }
        anchor
    }
}

impl<'a, Message: 'static> Compose<Message> for Popover<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}

impl<'a, Message: 'static> From<Popover<'a, Message>> for Slot<'a, Message> {
    fn from(popover: Popover<'a, Message>) -> Self {
        Slot::new(popover)
    }
}

impl<'a, Message: 'static> From<Popover<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(popover: Popover<'a, Message>) -> Self {
        component::component(popover).boxed()
    }
}
