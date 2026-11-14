//! Tabs for switching between panels. The content closure receives a [`Handle`] for arranging
//! the tab list, triggers, and panels.
//!
//! ```ignore
//! use icedcn::widget::{column, tabs};
//!
//! tabs(|t| {
//!     column![
//!         t.list([
//!             t.trigger("overview", "Overview"),
//!             t.trigger("analytics", "Analytics"),
//!         ]),
//!         t.content("overview", overview()),
//!         t.content("analytics", analytics()),
//!     ]
//!     .spacing(8)
//!     .into()
//! })
//! .default_value("overview")
//! // Controlled: .value(&self.tab).on_value_change(Message::TabChanged)
//! ```
//!
//! Only the active panel is mounted. Switching tabs resets state kept in the outgoing panel’s
//! widget tree.

pub(crate) mod list;

use crate::column::Column;
use iced_core::Length::Fill;
use iced_core::{Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::{self, Space};
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;
use icedcn_core::theme::container;
use icedcn_core::theme::tabs;

use crate::Size;
use crate::button::button;
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};
use crate::row::Row;

/// A set of panels, one shown at a time, chosen by its triggers.
pub struct Tabs<'a, Message> {
    body: Body<'a, Message>,
    value: Option<String>,
    default_value: String,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    width: Length,
    orientation: Orientation,
    line: bool,
    list_width: Length,
    rounded: bool,
    inherited: Inherited,
}

/// The closure that builds a [`Tabs`]'s content.
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// What a [`Tabs`]'s content closure receives: the selected value, and the
/// builders for the list, its triggers and the panels.
#[derive(Debug, Clone)]
pub struct Handle {
    value: String,
    list_width: Length,
    rounded: bool,
    orientation: Orientation,
    line: bool,
}

/// List layout and arrow-key direction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// The signal a trigger records when pressed: the value it selects.
#[derive(Debug, Clone)]
struct Select(String);

/// The tabs' private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// A trigger selected a value.
    Select(String),
    /// The content published an app message.
    App(Message),
}

/// The selected value of uncontrolled tabs.
#[derive(Debug, Default)]
pub struct State {
    value: Option<String>,
}

/// Create uncontrolled [`Tabs`] whose content is built by `body`.
pub fn tabs<'a, Message>(body: impl Fn(&Handle) -> Slot<'a, Message> + 'a) -> Tabs<'a, Message> {
    Tabs {
        body: Box::new(body),
        value: None,
        default_value: String::new(),
        on_value_change: None,
        width: Length::Shrink,
        list_width: Length::Shrink,
        rounded: false,
        orientation: Orientation::Horizontal,
        line: false,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Tabs<'a, Message> {
    /// Control the tabs: `value` is always the selected one, and a trigger
    /// only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The value uncontrolled tabs start on.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = value.into();
        self
    }

    /// Set the message published with the new value when a trigger selects
    /// another tab.
    pub fn on_value_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Orient the list vertically or horizontally.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Display the active tab with a line indicator instead of a filled pill.
    pub fn line(mut self, line: bool) -> Self {
        self.line = line;
        self
    }

    /// Set the width of the tabs.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the tab strip width. Horizontal triggers share any extra width equally.
    pub fn list_width(mut self, width: impl Into<Length>) -> Self {
        self.list_width = width.into();
        self
    }

    /// Round the list and its triggers into pills.
    pub fn rounded(mut self) -> Self {
        self.rounded = true;
        self
    }

    fn selected<'s>(&'s self, state: &'s State) -> &'s str {
        self.value
            .as_deref()
            .or(state.value.as_deref())
            .unwrap_or(&self.default_value)
    }
}

impl Handle {
    /// The selected value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// The `TabsList`: the muted pill holding the triggers.
    pub fn list<'a, Message: 'a, T: Into<Slot<'a, Message>>>(
        &self,
        triggers: impl IntoIterator<Item = T>,
    ) -> Slot<'a, Message> {
        let gap = if self.line { 4 } else { 0 };
        let children = triggers.into_iter().map(Into::into);
        let content: Slot<'a, Message> = match self.orientation {
            Orientation::Horizontal => Row::with_children(children)
                .spacing(gap)
                .width(self.list_width)
                .into(),
            Orientation::Vertical => Column::with_children(children)
                .spacing(gap)
                .width(self.list_width)
                .into(),
        };
        let mut content = Container::new(content).width(self.list_width).padding(3);
        if self.orientation == Orientation::Horizontal && !self.line {
            content = content.center_y(32);
        }
        if !self.line {
            let rounded = self.rounded;
            content = content.style(move |theme: &Theme| {
                let mut style = tabs::list(theme);
                if rounded {
                    style.border.radius = 18.0.into();
                }
                style
            });
        }
        Slot::from(
            list::List {
                content: Slot::from(content).compose(),
                orientation: Some(self.orientation),
            }
            .boxed(),
        )
    }

    /// A trigger that selects `value` and can be individually disabled.
    pub fn trigger<'a, Message: Clone + 'a>(
        &self,
        value: impl Into<String>,
        label: impl Into<Slot<'a, Message>>,
    ) -> Trigger<'a, Message> {
        let value = value.into();
        let active = value == self.value;
        let line = self.line;
        let rounded = self.rounded;
        let mut label = label.into();
        let inherited = Inherited::default().with_size(Size::Default);
        label.inherit(&if self.rounded {
            inherited.with_line_height(20.0)
        } else {
            inherited
        });
        let style: iced_widget::button::StyleFn<'a, Theme> = Box::new(move |theme, status| {
            let mut style = tabs::trigger(theme, status, active);
            if rounded {
                style.shadow = Default::default();
                style.border.color = icedcn_core::Color::TRANSPARENT;
            }
            if line {
                style.background = None;
                style.border.width = 0.0;
                style.shadow = Default::default();
            }
            style.into()
        });
        let mut button = button(label).size(Size::Xs).translate(false).style(style);
        if self.list_width != Length::Shrink {
            button = button.width(Fill);
        }
        if self.rounded {
            button = button.height(25).padding([2, 6]).rounded();
        }
        Trigger {
            button,
            value,
            disabled: false,
            active,
            line,
            orientation: self.orientation,
        }
    }

    /// A `TabsContent` panel, shown only while `value` is selected.
    pub fn content<'a, Message: 'a>(
        &self,
        value: &str,
        child: impl Into<Slot<'a, Message>>,
    ) -> Slot<'a, Message> {
        if value == self.value {
            child.into()
        } else {
            Slot::from(iced_widget::Space::new())
        }
    }
}

/// A trigger button that records [`Select`] when pressed.
pub struct Trigger<'a, Message> {
    button: crate::Button<'a, Message>,
    value: String,
    disabled: bool,
    active: bool,
    line: bool,
    orientation: Orientation,
}

impl<Message> Trigger<'_, Message> {
    /// Exclude this trigger from pointer and keyboard selection.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a, Message: Clone + 'a> From<Trigger<'a, Message>> for Slot<'a, Message> {
    fn from(trigger: Trigger<'a, Message>) -> Self {
        Slot::new(trigger)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Trigger<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
        Compose::inherit(&mut self.button, from);
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Trigger {
            button,
            value,
            disabled,
            active,
            line,
            orientation,
        } = *self;
        let button: Slot<'a, Message> = button.disabled(disabled).into();
        let content: Slot<'a, Message> = if line {
            let indicator =
                Container::new(Space::new()).style(move |theme: &Theme| container::Style {
                    background: active.then(|| theme.neutral.foreground.into()),
                    ..container::Style::default()
                });
            match orientation {
                Orientation::Horizontal => crate::column![button, indicator.width(Fill).height(2)]
                    .spacing(2)
                    .into(),
                Orientation::Vertical => crate::row![button, indicator.width(2).height(Fill)]
                    .spacing(2)
                    .into(),
            }
        } else {
            button
        };
        let marker = marker::marker(content.compose(), Select(value), Press::Captured);
        if disabled {
            marker.into()
        } else {
            marker.control(active).into()
        }
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Tabs<'a, Message> {
    type State = State;
    type Event = Event<Message>;

    fn diff(&mut self, state: &mut State) {
        if self.value.is_none() && state.value.is_none() {
            state.value = Some(self.default_value.clone());
        }
    }

    fn update(
        &self,
        state: &mut State,
        event: Event<Message>,
        _renderer: &Renderer,
    ) -> Option<Message> {
        match event {
            Event::App(message) => Some(message),
            Event::Select(value) => {
                if value == self.selected(state) {
                    return None;
                }

                if self.value.is_none() {
                    state.value = Some(value.clone());
                }

                self.on_value_change
                    .as_ref()
                    .map(|on_change| on_change(value))
            }
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let handle = Handle {
            value: self.selected(state).to_owned(),
            list_width: self.list_width,
            rounded: self.rounded,
            orientation: self.orientation,
            line: self.line,
        };

        let mut body = (self.body)(&handle);
        body.inherit(&self.inherited);

        let body = iced_widget::container(body.compose().map(Event::App)).width(self.width);

        marker::owner(body.boxed(), |Select(value)| Event::Select(value))
    }
}

impl<'a, Message: 'static> Compose<Message> for Tabs<'a, Message> {
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

impl<'a, Message: 'static> From<Tabs<'a, Message>> for Slot<'a, Message> {
    fn from(tabs: Tabs<'a, Message>) -> Self {
        Slot::new(tabs)
    }
}

impl<'a, Message: 'static> From<Tabs<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(tabs: Tabs<'a, Message>) -> Self {
        component::component(tabs).boxed()
    }
}
