//! A group of radio controls with one selected value.
//!
//! An iced `Component` that owns the selected value. Its content is a
//! closure receiving a [`Handle`], whose [`item`](Handle::item) builds each
//! `RadioGroupItem` wherever the layout puts it; a [`field`](crate::field)
//! around an item links its label:
//!
//! ```ignore
//! use icedcn::widget::{column, field, radio_group};
//! use icedcn::Horizontal;
//!
//! radio_group(|g| {
//!     column![
//!         field(g.item("default")).orientation(Horizontal).label("Default"),
//!         field(g.item("comfortable")).orientation(Horizontal).label("Comfortable"),
//!     ]
//!     .spacing(8)
//!     .into()
//! })
//! .default_value("comfortable")
//! // Controlled: .value(&self.density).on_value_change(Message::Density)
//! ```

use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Border, Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::radio::{Status, Style};
use iced_widget::{Renderer, Space, Widget, container};
use icedcn_core::Theme;
use icedcn_core::theme::radio;
use std::convert::Infallible;
use std::rc::Rc;

use crate::button::widget;
use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::marker::{self, Press};
use crate::tabs::list::List;

/// A set of radio items, one checked at a time.
pub struct Group<'a, Message> {
    body: Body<'a, Message>,
    value: Option<String>,
    default_value: String,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
    inherited: Inherited,
}

/// The closure that builds a [`Group`]'s content.
type Body<'a, Message> = Box<dyn Fn(&Handle) -> Slot<'a, Message> + 'a>;

/// What a [`Group`]'s content closure receives: the selected value, and
/// the builder for its items.
#[derive(Debug, Clone)]
pub struct Handle {
    value: String,
    disabled: bool,
}

/// The signal an item records when chosen: its value.
#[derive(Debug, Clone)]
struct Select(String);

/// The group's private events.
#[derive(Debug, Clone)]
pub enum Event<Message> {
    /// An item was chosen.
    Select(String),
    /// The content published an app message.
    App(Message),
}

/// The selected value of an uncontrolled group.
#[derive(Debug, Default)]
pub struct State {
    value: Option<String>,
}

/// Create an uncontrolled [`Group`] whose content is built by `body`.
pub fn group<'a, Message>(body: impl Fn(&Handle) -> Slot<'a, Message> + 'a) -> Group<'a, Message> {
    Group {
        body: Box::new(body),
        value: None,
        default_value: String::new(),
        on_value_change: None,
        disabled: false,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Group<'a, Message> {
    /// Control the group: `value` is always the checked item, and choosing
    /// another only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The value an uncontrolled group starts on.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = value.into();
        self
    }

    /// Set the message published with the new value when another item is
    /// chosen.
    pub fn on_value_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Disable every item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    /// A `RadioGroupItem` for `value`.
    pub fn item(&self, value: impl Into<String>) -> Item {
        let value = value.into();

        Item {
            is_checked: value == self.value,
            invalid: false,
            value,
            disabled: self.disabled,
            style: None,
        }
    }
}

/// One radio of a [`Group`]: a 16px circle, filled while checked.
#[derive(Clone)]
pub struct Item {
    value: String,
    is_checked: bool,
    invalid: bool,
    disabled: bool,
    style: Option<Paint>,
}

/// A custom look for an [`Item`].
type Paint = Rc<dyn Fn(&Theme, Status) -> Style>;

impl Item {
    /// Indicate an invalid field with Nova's destructive border and validation ring.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Disable this item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled |= disabled;
        self
    }

    /// Paint the circle and its indicator with `style` instead of Nova's;
    /// a disabled item still dims it to half opacity.
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'static) -> Self {
        self.style = Some(Rc::new(style));
        self
    }
}

impl<Message: 'static> Compose<Message> for Item {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Item {
            value,
            is_checked,
            invalid,
            disabled,
            style,
        } = *self;

        let status = Status::Active {
            is_selected: is_checked,
        };
        let paint = Rc::new(move |theme: &Theme| {
            let style = match &style {
                Some(style) => style(theme, status),
                None => radio::default(theme, status),
            };
            if disabled {
                radio::dimmed(theme, style)
            } else {
                style
            }
        });

        // `size-2 rounded-full bg-primary-foreground`.
        let dot: Slot<'b, Infallible> = if is_checked {
            let paint = paint.clone();
            Container::new(Space::new())
                .width(8)
                .height(8)
                .class(Box::new(move |theme: &Theme| {
                    container::Style::default()
                        .background(paint(theme).dot_color)
                        .border(Border::default().rounded(4))
                }) as container::StyleFn<'static, Theme>)
                .into()
        } else {
            Slot::from(Space::new())
        };

        let centered = Container::new(dot)
            .width(16)
            .height(16)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);
        let hit = widget::Button::new(Element::from(centered))
            .width(16)
            .height(16)
            .padding(0)
            .translate(false)
            .disabled(disabled)
            .style(Box::new(move |theme: &Theme, status| {
                let style = paint(theme);
                let focused = matches!(status, widget::Status::Focused { .. });
                let dark = theme.neutral.base.l < 0.5;
                let destructive = theme.destructive.base;
                let ring = if invalid {
                    Some(destructive.scale_alpha(if dark { 0.4 } else { 0.2 }))
                } else if focused {
                    Some(theme.neutral.ring.alpha(0.5))
                } else {
                    None
                };

                widget::Style {
                    background: Some(style.background),
                    border: Border::default().width(1).rounded(8).color(
                        if invalid && !is_checked {
                            destructive.scale_alpha(if dark { 0.5 } else { 1. }).into()
                        } else if focused && !invalid {
                            theme.neutral.ring.into()
                        } else {
                            style.border_color
                        },
                    ),
                    ring: ring.map(|color| Border::default().width(3).rounded(11).color(color)),
                    ..widget::Style::default()
                }
            }) as widget::StyleFn<'b, Theme>);
        let hit = hit.boxed().map(|never| match never {}).boxed();
        if disabled {
            hit
        } else {
            marker::marker(hit, Select(value), Press::Captured)
                .control(is_checked)
                .into()
        }
    }
}

impl<'a, Message: 'static> From<Item> for Slot<'a, Message> {
    fn from(item: Item) -> Self {
        Slot::new(item)
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Group<'a, Message> {
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
            disabled: self.disabled,
        };

        let mut body = (self.body)(&handle);
        body.inherit(&self.inherited);

        let body = iced_widget::container(body.compose().map(Event::App)).width(Length::Shrink);

        let list = List {
            content: body.boxed(),
            orientation: None,
            single_tab_stop: true,
        };
        marker::owner(list.boxed(), |Select(value)| Event::Select(value))
    }
}

impl<'a, Message: 'static> Compose<Message> for Group<'a, Message> {
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

impl<'a, Message: 'static> From<Group<'a, Message>> for Slot<'a, Message> {
    fn from(group: Group<'a, Message>) -> Self {
        Slot::new(group)
    }
}

impl<'a, Message: 'static> From<Group<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(group: Group<'a, Message>) -> Self {
        component::component(group).boxed()
    }
}
