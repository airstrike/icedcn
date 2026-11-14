//! A searchable list of options with an input and popup.
//!
//! An input that filters a list of items anchored below it. Typing or
//! focusing the input opens the list, which shows the items containing the
//! typed text (ignoring case) or `ComboboxEmpty`'s message when none do;
//! choosing one fills the input with it and closes the list. An iced
//! `Component` that owns the typed text, the open state and, uncontrolled,
//! the value:
//!
//! ```ignore
//! use icedxdy::widget::combobox;
//!
//! combobox(["Cargo", "Clippy", "Miri", "Rustfmt"])
//!     .placeholder("Select a tool")
//!     .width(256)
//! // Controlled: .value(self.tool.clone()).on_value_change(Message::Tool)
//! ```
//!
//! The list is as wide as the input, non-modal (the page keeps the
//! pointer), and closes on `Escape` or a press outside it.

pub(crate) mod keys;
use crate::menu::list::List;
use crate::{button, row};
use iced_core::Length::Fill;
use iced_core::keyboard::key::Named;
use iced_core::{Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::text::Text;
use iced_widget::{Renderer, Space, Widget, anchored};
use icedxdy_core::Theme;
use icedxdy_core::theme::popover::content;
use icedxdy_core::theme::text_input::Invalid;
use keys::Keys;

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::input_group::{self, input_group};
use crate::{column, input, menu, scrollable, text};

/// A searchable option; its stable value can differ from the text displayed.
#[derive(Clone, Debug)]
pub struct Item {
    value: String,
    label: String,
    description: Option<String>,
    group: Option<String>,
}
/// Make an option whose label can be customized independently of its value.
pub fn item(value: impl Into<String>, label: impl Into<String>) -> Item {
    Item {
        value: value.into(),
        label: label.into(),
        description: None,
        group: None,
    }
}
impl Item {
    /// Secondary text displayed below the option's label.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Group consecutive options under a heading.
    pub fn group(mut self, label: impl Into<String>) -> Self {
        self.group = Some(label.into());
        self
    }
}
/// An input choosing one of its items by filtering them as you type.
pub struct Combobox<'a, Message> {
    items: Vec<Item>,
    auto_highlight: bool,
    clear: bool,
    invalid: bool,
    popup: bool,
    multiple: bool,
    selected: Option<Vec<String>>,
    defaults: Vec<String>,
    on_selection_change: Option<Box<dyn Fn(Vec<String>) -> Message + 'a>>,
    leading: Option<Box<dyn Fn() -> Text<'a, Theme> + 'a>>,
    placeholder: String,
    empty: String,
    value: Option<String>,
    default_value: Option<String>,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
    width: Length,
    icon: Option<Box<dyn Fn() -> iced_widget::Text<'a, Theme> + 'a>>,
    inherited: Inherited,
}

/// The combobox's private events.
#[derive(Debug, Clone)]
pub enum Event {
    /// The input was edited.
    Query(String),
    /// The input took focus.
    Focus,
    /// The list asked to open or close.
    Set(bool),
    /// An item was chosen.
    Choose(String),
    /// Clear the selection.
    Clear,
    /// Move the highlighted option.
    Step(bool),
    /// Pick the highlighted option.
    Submit,
}

/// What a combobox holds: the typed text, whether the list is open and,
/// uncontrolled, the value.
#[derive(Debug, Default)]
pub struct State {
    query: Option<String>,
    selection: Option<Vec<String>>,
    highlight: Option<usize>,
    open: bool,
    value: Option<String>,
}

/// Create an uncontrolled [`Combobox`] over `items`.
pub fn combobox<'a, Message>(
    items: impl IntoIterator<Item = impl Into<String>>,
) -> Combobox<'a, Message> {
    options(items.into_iter().map(|value| {
        let value = value.into();
        item(value.clone(), value)
    }))
}
/// Create a combobox with structured values, descriptions or grouped options.
pub fn options<'a, Message>(items: impl IntoIterator<Item = Item>) -> Combobox<'a, Message> {
    Combobox {
        items: items.into_iter().collect(),
        auto_highlight: false,
        clear: false,
        invalid: false,
        popup: false,
        multiple: false,
        selected: None,
        defaults: Vec::new(),
        on_selection_change: None,
        leading: None,
        placeholder: String::new(),
        empty: "No items found.".to_owned(),
        value: None,
        default_value: None,
        on_value_change: None,
        disabled: false,
        width: Length::Fixed(256.0),
        icon: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message> Combobox<'a, Message> {
    /// Set the input's placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the message shown when nothing matches (`ComboboxEmpty`).
    pub fn empty(mut self, empty: impl Into<String>) -> Self {
        self.empty = empty.into();
        self
    }

    /// Control the combobox: it holds exactly `value`, and choosing an item
    /// only publishes [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// The value an uncontrolled combobox starts with.
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Set the message published with the chosen item.
    pub fn on_value_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_value_change = Some(Box::new(on_change));
        self
    }

    /// Show the first matching item as the active option when filtering.
    pub fn auto_highlight(mut self, enabled: bool) -> Self {
        self.auto_highlight = enabled;
        self
    }
    /// Show a clear button while a value is selected.
    pub fn clearable(mut self, enabled: bool) -> Self {
        self.clear = enabled;
        self
    }
    /// Style the control as invalid.
    pub fn invalid(mut self, enabled: bool) -> Self {
        self.invalid = enabled;
        self
    }
    /// Place the search input in the popup, opened by a button.
    pub fn popup(mut self, enabled: bool) -> Self {
        self.popup = enabled;
        self
    }
    /// Show selected items as removable chips.
    pub fn multiple(mut self, enabled: bool) -> Self {
        self.multiple = enabled;
        self
    }
    /// Initial selections for an uncontrolled multiple combobox.
    pub fn default_values(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.defaults = values.into_iter().map(Into::into).collect();
        self.multiple = true;
        self
    }
    /// Controlled selections for a multiple combobox.
    pub fn values(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.selected = Some(values.into_iter().map(Into::into).collect());
        self.multiple = true;
        self
    }
    /// Receive every selected value when a chip or option is pressed.
    pub fn on_values_change(mut self, f: impl Fn(Vec<String>) -> Message + 'a) -> Self {
        self.on_selection_change = Some(Box::new(f));
        self
    }
    /// Add an icon to the leading edge of the input group.
    pub fn leading_icon(mut self, icon: impl Fn() -> Text<'a, Theme> + 'a) -> Self {
        self.leading = Some(Box::new(icon));
        self
    }

    /// Disable the combobox.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the input's width (256px by default).
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Replace the built-in chevron at the end of the input.
    pub fn icon(mut self, icon: impl Fn() -> iced_widget::Text<'a, Theme> + 'a) -> Self {
        self.icon = Some(Box::new(icon));
        self
    }

    fn selected<'s>(&'s self, state: &'s State) -> &'s [String] {
        self.selected
            .as_deref()
            .or(state.selection.as_deref())
            .unwrap_or(&self.defaults)
    }
    fn label<'s>(&'s self, value: &'s str) -> &'s str {
        self.items
            .iter()
            .find(|item| item.value == value)
            .map_or(value, |item| item.label.as_str())
    }
    fn matches<'s>(&'s self, state: &State) -> impl Iterator<Item = &'s Item> {
        let needle = state.query.as_deref().unwrap_or("").to_lowercase();
        self.items
            .iter()
            .filter(move |item| item.label.to_lowercase().contains(&needle))
    }
    fn choose(&self, state: &mut State, value: String) -> Option<Message> {
        state.query = None;
        state.highlight = None;
        if self.multiple {
            let mut selected = self.selected(state).to_vec();
            if let Some(index) = selected.iter().position(|v| *v == value) {
                selected.remove(index);
            } else {
                selected.push(value);
            }
            if self.selected.is_none() {
                state.selection = Some(selected.clone());
            }
            self.on_selection_change.as_ref().map(|f| f(selected))
        } else {
            state.open = false;
            if self.value.is_none() {
                state.value = Some(value.clone());
            }
            self.on_value_change.as_ref().map(|f| f(value))
        }
    }
    fn current<'s>(&'s self, state: &'s State) -> Option<&'s str> {
        self.value
            .as_deref()
            .or(state.value.as_deref())
            .or(self.default_value.as_deref())
    }
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer>
    for Combobox<'a, Message>
{
    type State = State;
    type Event = Event;

    fn update(&self, state: &mut State, event: Event, _renderer: &Renderer) -> Option<Message> {
        match event {
            Event::Query(query) => {
                state.query = Some(query);
                state.open = true;
                state.highlight = self.auto_highlight.then_some(0);
                None
            }
            Event::Focus => {
                state.open = true;
                state.highlight = self.auto_highlight.then_some(0);
                None
            }
            Event::Set(open) => {
                state.open = open;
                state.query = None;
                state.highlight = (open && self.auto_highlight).then_some(0);
                None
            }
            Event::Step(reverse) => {
                state.open = true;
                let n = self.matches(state).count();
                state.highlight = if n == 0 {
                    None
                } else {
                    Some(match (state.highlight, reverse) {
                        (Some(i), true) => (i + n - 1) % n,
                        (Some(i), false) => (i + 1) % n,
                        (None, true) => n - 1,
                        (None, false) => 0,
                    })
                };
                None
            }
            Event::Submit => {
                let value = self
                    .matches(state)
                    .nth(state.highlight?)
                    .map(|item| item.value.clone())?;
                self.choose(state, value)
            }
            Event::Choose(value) => self.choose(state, value),
            Event::Clear => {
                state.query = None;
                state.highlight = None;
                state.open = false;
                if self.value.is_none() {
                    state.value = Some(String::new());
                }
                self.on_value_change.as_ref().map(|f| f(String::new()))
            }
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        let current = self.current(state).map(str::to_owned);
        let choice = current
            .as_deref()
            .map(|value| self.label(value))
            .unwrap_or("");
        let query = state
            .query
            .as_deref()
            .unwrap_or(if self.multiple || self.popup {
                ""
            } else {
                choice
            });
        let open = state.open && !self.disabled;
        let make_input = || {
            let mut group = input_group(
                input(self.placeholder.clone())
                    .value(query.to_owned())
                    .on_input(Event::Query)
                    .on_focus(Event::Focus)
                    .disabled(self.disabled),
            );
            if self.invalid {
                group = group.style(Invalid);
            }
            if let Some(leading) = &self.leading {
                group = group.addon(input_group::addon(leading().size(16)));
            }
            if self.multiple && !self.selected(state).is_empty() {
                let chips = self.selected(state).iter().map(|value| {
                    Slot::from(
                        input_group::button(row![self.label(value).to_owned(), "×"].spacing(4))
                            .style(button::Secondary)
                            .on_press(Event::Choose(value.clone())),
                    )
                });
                group = group.addon(input_group::addon(
                    row::Row::with_children(chips).spacing(2),
                ));
            } else if !self.popup && !self.multiple {
                let end = if self.clear && !choice.is_empty() {
                    input_group::button("×")
                        .square()
                        .on_press(Event::Clear)
                        .into()
                } else {
                    let chevron: Slot<'a, Event> = match &self.icon {
                        Some(icon) => icon().size(16).into(),
                        None => text::muted("⌄").into(),
                    };
                    Slot::from(
                        input_group::button(chevron)
                            .square()
                            .on_press(Event::Set(!open)),
                    )
                };
                group = group.addon(input_group::addon(end).align(input_group::Align::InlineEnd));
            }
            let input = Slot::from(group.disabled(self.disabled)).compose();
            Keys {
                content: input,
                on_key: Box::new(move |key| match key {
                    Named::ArrowDown => Some(Event::Step(false)),
                    Named::ArrowUp => Some(Event::Step(true)),
                    Named::Enter if open => Some(Event::Submit),
                    _ => None,
                }),
            }
            .boxed()
        };
        let mut field: Slot<'a, Event> = if self.popup {
            button(if choice.is_empty() {
                self.placeholder.clone()
            } else {
                choice.to_owned()
            })
            .width(self.width)
            .style(button::Outline)
            .disabled(self.disabled)
            .on_press(Event::Set(!open))
            .into()
        } else {
            Container::new(make_input()).width(self.width).into()
        };
        field.inherit(&self.inherited);
        let is_open = open;
        let list: Element<'a, Event, Theme, Renderer> = if is_open {
            let mut matches = Vec::new();
            let mut group = None;
            for (index, item) in self.matches(state).enumerate() {
                if item.group.as_deref() != group {
                    if group.is_some() {
                        matches.push(menu::separator());
                    }
                    if let Some(name) = &item.group {
                        matches.push(menu::label(name.clone()));
                    }
                    group = item.group.as_deref();
                }
                let chosen = if self.multiple {
                    self.selected(state).contains(&item.value)
                } else {
                    current.as_deref() == Some(item.value.as_str())
                };
                let label: Slot<'a, Event> = if let Some(detail) = &item.description {
                    crate::column![
                        text::small(item.label.clone()).medium(),
                        text::muted(detail.clone())
                    ]
                    .spacing(2)
                    .into()
                } else {
                    text::small(item.label.clone()).into()
                };
                let value = item.value.clone();
                matches.push(
                    menu::checkbox_item(label, chosen, move |_| Event::Choose(value.clone()))
                        .highlighted(state.highlight == Some(index))
                        .closes(false)
                        .into(),
                );
            }
            let body: Slot<'a, Event> = if matches.is_empty() {
                // `ComboboxEmpty`: `py-2 text-center text-sm text-muted-foreground`.
                Container::new(text::muted(self.empty.clone()))
                    .width(Fill)
                    .center_x(Fill)
                    .padding([8, 0])
                    .into()
            } else {
                column::Column::with_children(matches).width(Fill).into()
            };

            let results = scrollable(Container::new(body).width(Fill).padding(4))
                .width(Fill)
                .height(Length::Shrink);
            let panel: Slot<'a, Event> = if self.popup {
                crate::column![make_input(), results].width(Fill).into()
            } else {
                results.into()
            };
            let panel = Container::new(panel)
                .width(Fill)
                .max_height(252)
                .style(content);
            if self.popup {
                List {
                    content: Slot::from(panel).compose(),
                    horizontal: None,
                    autofocus: true,
                    on_move: None,
                }
                .boxed()
            } else {
                Slot::from(panel).compose()
            }
        } else {
            Space::new().boxed()
        };

        anchored(field.compose(), list)
            .side(anchored::Side::Bottom)
            .offset(6)
            .width(anchored::Width::AtLeastAnchor)
            .open(is_open)
            .on_toggle(Event::Set)
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for Combobox<'a, Message> {
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

impl<'a, Message: Clone + 'static> From<Combobox<'a, Message>> for Slot<'a, Message> {
    fn from(combobox: Combobox<'a, Message>) -> Self {
        Slot::new(combobox)
    }
}

impl<'a, Message: Clone + 'static> From<Combobox<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(combobox: Combobox<'a, Message>) -> Self {
        component::component(combobox).boxed()
    }
}
