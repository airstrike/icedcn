//! A menu anchored to the pointer.
//!
//! A menu opened at the pointer by a right press inside its trigger area,
//! over iced's [`anchored`](iced_widget::anchored) overlay anchored to that
//! point. Build the menu from [`menu`](crate::menu) parts; selecting an item
//! closes it, as do `Escape` and any press outside (the menu is modal).
//!
//! ```ignore
//! use icedcn::widget::{column, context_menu, menu, text};
//!
//! context_menu(
//!     |_| text("Right click here").into(),
//!     |_| {
//!         menu::group([
//!             menu::item("Back").shortcut("⌘[").into(),
//!             menu::item("Reload").on_select(Message::Reload).into(),
//!         ])
//!     },
//! )
//! .width(192)
//! ```

mod gesture;
use gesture::Trigger;
use iced_core::Length::Fill;
use iced_core::{Element, Length, Pixels, Point};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Space, Widget, anchored};
use icedcn_core::Theme;
use icedcn_core::theme::popover::content;

use crate::compose::{Compose, Inherited, View};
use crate::container::Container;
use crate::marker;
use crate::menu::list::List;
use crate::popover::{Handle, Signal};

/// A closure that builds part of a [`ContextMenu`].
type Body<'a, Message> = Box<dyn Fn(&Handle) -> View<'a, Message> + 'a>;

/// A menu opened at the pointer by a right press.
pub struct ContextMenu<'a, Message> {
    trigger: Body<'a, Message>,
    content: Body<'a, Message>,
    width: Option<f32>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    inherited: Inherited,
}

/// The context menu's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A right press or stationary touch opened the menu at this point in the trigger.
    OpenAt(Point),
    /// The menu asked to close.
    Close,
    /// The content published an app message.
    App(Message),
}

/// Where the menu is open inside its trigger area.
#[derive(Debug, Default)]
pub struct State {
    open_at: Option<Point>,
}

/// Create a [`ContextMenu`]: `trigger` builds the area that opens it,
/// `content` the menu.
pub fn context_menu<'a, Message>(
    trigger: impl Fn(&Handle) -> View<'a, Message> + 'a,
    content: impl Fn(&Handle) -> View<'a, Message> + 'a,
) -> ContextMenu<'a, Message> {
    ContextMenu {
        trigger: Box::new(trigger),
        content: Box::new(content),
        width: None,
        on_open_change: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> ContextMenu<'a, Message> {
    /// Set a fixed menu width. By default the menu fits its items with a 144-pixel minimum.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into().0);
        self
    }

    /// Set the message published when the menu opens or closes.
    pub fn on_open_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_change));
        self
    }
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer>
    for ContextMenu<'a, Message>
{
    type State = State;
    type Event = Event<Message>;

    fn update(
        &self,
        state: &mut State,
        event: Event<Message>,
        _renderer: &Renderer,
    ) -> Option<Message> {
        let open = match event {
            Event::App(message) => return Some(message),
            Event::OpenAt(point) => {
                state.open_at = Some(point);
                true
            }
            Event::Close => {
                let _ = state.open_at.take()?;
                false
            }
        };

        self.on_open_change
            .as_ref()
            .map(|on_change| on_change(open))
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let handle = Handle::new(state.open_at.is_some());

        let mut area = (self.trigger)(&handle);
        area.inherit(&self.inherited);
        let area = Trigger {
            content: area.compose().map(Event::App).boxed(),
            open: Event::OpenAt,
        };

        let menu: Element<'a, Event<Message>, Theme, Renderer> = if state.open_at.is_some() {
            let mut body = (self.content)(&handle);
            body.inherit(&self.inherited);

            // `min-w-36 p-1 rounded-lg bg-popover shadow-md ring-1`.
            let panel = Container::new(body)
                .padding(4)
                .width(self.width.map_or(Fill, Length::Fixed))
                .style(content);

            let body = marker::owner(
                View::from(panel).compose().map(Event::App).boxed(),
                |signal| match signal {
                    Signal::Toggle | Signal::Close => Event::Close,
                },
            )
            .boxed();
            List {
                content: body,
                horizontal: Some(false),
                autofocus: true.into(),
                on_move: None,
            }
            .boxed()
        } else {
            Space::new().boxed()
        };

        // Align the menu’s top edge with the pointer and leave a two-pixel gap to its right.
        anchored(area.boxed(), menu)
            .at(state.open_at.unwrap_or(Point::ORIGIN))
            .side(anchored::Side::Right)
            .align(anchored::Align::Start)
            .offset(2)
            .min_width(144)
            .modal(true)
            .open(state.open_at.is_some())
            .on_toggle(|_| Event::Close)
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for ContextMenu<'a, Message> {
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

impl<'a, Message: Clone + 'static> From<ContextMenu<'a, Message>> for View<'a, Message> {
    fn from(menu: ContextMenu<'a, Message>) -> Self {
        View::new(menu)
    }
}

impl<'a, Message: Clone + 'static> From<ContextMenu<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(menu: ContextMenu<'a, Message>) -> Self {
        component::component(menu).boxed()
    }
}
