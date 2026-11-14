//! A searchable list of options with an input and popup.
//!
//! An input that filters a list of items anchored below it. Typing or
//! focusing the input opens the list, which shows the items containing the
//! typed text (ignoring case) or the configured empty message when none do;
//! choosing one fills the input with it and closes the list. An iced
//! `Component` that owns the typed text, the open state and, uncontrolled,
//! the value:
//!
//! ```ignore
//! use icedcn::widget::combobox;
//!
//! combobox(["Cargo", "Clippy", "Miri", "Rustfmt"])
//!     .placeholder("Select a tool")
//!     .width(256)
//! // Controlled: .value(self.tool.clone()).on_value_change(Message::Tool)
//! ```
//!
//! The list is as wide as the input, non-modal (the page keeps the
//! pointer), and closes on `Escape` or a press outside it.

mod behavior;
mod icon;
pub(crate) mod keys;
use crate::Weight;
use crate::button::widget;
use crate::input_group::widget::Surface;
use crate::marker::{self, Press};
use crate::{button, row};
use behavior::Behavior;
use iced_core::Length::Fill;
use iced_core::keyboard::key::Named;
use iced_core::widget::Id;
use iced_core::{Element, Length, Padding, Pixels};
use iced_widget::button::Status;
use iced_widget::component::{self, Component};
use iced_widget::text::LineHeight;
use iced_widget::{Renderer, Space, Widget, anchored};
use icedcn_core::Theme;
use icedcn_core::theme::input_group::control;
use icedcn_core::theme::menu::item;
use icedcn_core::theme::popover::content;
use icedcn_core::theme::text_input::Invalid;
use icedcn_core::theme::{self, text_input};
use icon::Icon;
use keys::Keys;
use widget::Button;

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;
use crate::input_group::{self, input_group};
use crate::{column, input, menu, scrollable, text};

/// A searchable option; its stable value can differ from the text displayed.
#[derive(Clone, Debug)]
pub struct Item {
    /// Stable value published when this option is chosen.
    pub value: String,
    /// Searchable display label.
    pub label: String,
    /// Optional secondary text.
    pub description: Option<String>,
    /// Optional group heading.
    pub group: Option<String>,
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

type Render<'a, T, Message> = dyn Fn(&T) -> Slot<'a, Message> + 'a;

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
    leading: Option<Box<dyn Fn() -> Slot<'a, Message> + 'a>>,
    format_item: Option<Box<Render<'a, Item, Message>>>,
    trigger: Option<Box<Render<'a, str, Message>>>,
    content_width: Option<f32>,
    align_offset: f32,
    separators: bool,
    placeholder: String,
    empty: String,
    value: Option<String>,
    default_value: Option<String>,
    on_value_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    disabled: bool,
    width: Option<Length>,
    icon: Option<Box<dyn Fn() -> Slot<'a, Message> + 'a>>,
    inherited: Inherited,
}

/// The combobox's private events.
#[derive(Debug, Clone)]
pub enum Event<Message = ()> {
    /// A message from custom content.
    App(Message),
    /// Home/End move to the first/last result.
    Edge(bool),
    /// Dismiss without restoring focus (Tab or outside pointer).
    Dismiss,
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
    initialized: bool,
    focus_epoch: u64,
    visible: Vec<usize>,
    ids: Vec<Id>,
    scroll: Option<Id>,
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
        format_item: None,
        trigger: None,
        content_width: None,
        align_offset: 0.0,
        separators: true,
        placeholder: String::new(),
        empty: "No items found.".to_owned(),
        value: None,
        default_value: None,
        on_value_change: None,
        disabled: false,
        width: None,
        icon: None,
        inherited: Inherited::default(),
    }
}

impl<'a, Message: 'a> Combobox<'a, Message> {
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
    pub fn leading_icon<Content: Into<Slot<'a, Message>>>(
        mut self,
        icon: impl Fn() -> Content + 'a,
    ) -> Self {
        self.leading = Some(Box::new(move || icon().into()));
        self
    }

    /// Disable the combobox.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the field width. Inline inputs default to 20 characters wide and popup triggers to 256 pixels.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Replace the built-in chevron at the end of the input.
    pub fn icon<Content: Into<Slot<'a, Message>>>(
        mut self,
        icon: impl Fn() -> Content + 'a,
    ) -> Self {
        self.icon = Some(Box::new(move || icon().into()));
        self
    }

    fn selected<'s>(&'s self, state: &'s State) -> &'s [String] {
        self.selected
            .as_deref()
            .or(state.selection.as_deref())
            .unwrap_or(&[])
    }
    fn label<'s>(&'s self, value: &'s str) -> &'s str {
        self.items
            .iter()
            .find(|item| item.value == value)
            .map_or(value, |item| item.label.as_str())
    }
    /// Render an option's content through the ordinary Compose/Slot cascade.
    pub fn format_item<Content: Into<Slot<'a, Message>>>(
        mut self,
        render: impl Fn(&Item) -> Content + 'a,
    ) -> Self {
        self.format_item = Some(Box::new(move |item| render(item).into()));
        self
    }

    /// Use a custom as-child popup trigger. Its own handlers still fire.
    pub fn trigger<Content: Into<Slot<'a, Message>>>(
        mut self,
        render: impl Fn(&str) -> Content + 'a,
    ) -> Self {
        self.trigger = Some(Box::new(move |label| render(label).into()));
        self.popup = true;
        self
    }

    /// Override the popup width independently of the field.
    pub fn content_width(mut self, width: impl Into<Pixels>) -> Self {
        self.content_width = Some(width.into().0);
        self
    }

    /// Shift the popup along the anchor's horizontal axis.
    pub fn align_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.align_offset = offset.into().0;
        self
    }

    /// Insert separators between groups (true by default).
    pub fn separators(mut self, enabled: bool) -> Self {
        self.separators = enabled;
        self
    }

    fn filter(&self, state: &mut State) {
        let needle = state.query.as_deref().unwrap_or("").to_lowercase();
        state.visible = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                item.label.to_lowercase().contains(&needle).then_some(index)
            })
            .collect();
    }

    fn close(&self, state: &mut State, restore: bool) {
        state.open = false;
        state.query = None;
        state.highlight = None;
        if restore {
            state.focus_epoch += 1;
        }
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
            state.focus_epoch += 1;
            self.filter(state);
            self.on_selection_change.as_ref().map(|f| f(selected))
        } else {
            self.close(state, true);
            if self.value.is_none() {
                state.value = Some(value.clone());
            }
            self.on_value_change.as_ref().map(|f| f(value))
        }
    }
    fn current<'s>(&'s self, state: &'s State) -> Option<&'s str> {
        self.value.as_deref().or(state.value.as_deref())
    }
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer>
    for Combobox<'a, Message>
{
    type State = State;
    type Event = Event<Message>;

    fn diff(&mut self, state: &mut State) {
        if !state.initialized {
            state.initialized = true;
            state.value = self.default_value.clone();
            state.selection = Some(self.defaults.clone());
            state.scroll = Some(Id::unique());
        }
        if self.disabled {
            self.close(state, false);
        }
        state.ids.resize_with(self.items.len(), Id::unique);
        self.filter(state);
        if state
            .highlight
            .is_some_and(|index| index >= state.visible.len())
        {
            state.highlight = None;
        }
    }

    fn update(
        &self,
        state: &mut State,
        event: Event<Message>,
        _renderer: &Renderer,
    ) -> Option<Message> {
        if let Event::App(message) = event {
            return Some(message);
        }
        if self.disabled {
            return None;
        }
        match event {
            Event::App(_) => unreachable!("application messages are forwarded above"),
            Event::Dismiss => {
                self.close(state, false);
                None
            }
            Event::Edge(last) => {
                state.highlight = if state.visible.is_empty() {
                    None
                } else {
                    Some(if last { state.visible.len() - 1 } else { 0 })
                };
                None
            }
            Event::Query(query) => {
                state.query = Some(query);
                state.open = true;
                self.filter(state);
                state.highlight = (self.auto_highlight && !state.visible.is_empty()).then_some(0);
                None
            }
            Event::Focus => {
                state.open = true;
                self.filter(state);
                state.highlight = (self.auto_highlight && !state.visible.is_empty()).then_some(0);
                None
            }
            Event::Set(open) => {
                if open {
                    state.open = true;
                    state.query = None;
                    self.filter(state);
                } else {
                    self.close(state, true);
                }
                state.highlight =
                    (open && self.auto_highlight && !state.visible.is_empty()).then_some(0);
                None
            }
            Event::Step(reverse) => {
                if !state.open {
                    self.filter(state);
                }
                state.open = true;
                let n = state.visible.len();
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
                let index = *state.visible.get(state.highlight?)?;
                let value = self.items.get(index)?.value.clone();
                self.choose(state, value)
            }
            Event::Choose(value) => self.choose(state, value),
            Event::Clear => {
                state.query = None;
                state.highlight = None;
                self.close(state, true);
                if self.value.is_none() {
                    state.value = Some(String::new());
                }
                self.on_value_change.as_ref().map(|f| f(String::new()))
            }
        }
    }

    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
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
        let width = self.width.unwrap_or(if self.popup {
            Length::Fixed(256.0)
        } else {
            Length::Shrink
        });
        let inherited = self.inherited.without_join().with_disabled(self.disabled);
        let chevron = || -> Slot<'a, Event<Message>> {
            match &self.icon {
                Some(icon) => icon().map(Event::App),
                None => Container::new(Icon::Chevron.boxed())
                    .style(|theme| theme::container::Style {
                        text_color: Some(theme.muted.foreground),
                        ..Default::default()
                    })
                    .into(),
            }
        };
        let make_input = || {
            let input = input(self.placeholder.clone())
                .value(query.to_owned())
                .on_input(Event::Query)
                .on_focus(Event::Focus)
                .disabled(self.disabled);
            let mut field: Slot<'a, Event<Message>> = if self.multiple {
                let mut chips = Vec::new();
                for value in self.selected(state) {
                    let remove = button(Icon::Remove.boxed())
                        .style(button::Ghost)
                        .size(crate::Size::Xs)
                        .height(21)
                        .width(21)
                        .padding(0)
                        .translate(false)
                        .on_press(Event::Choose(value.clone()));
                    chips.push(
                        Container::new(
                            row![
                                text::small(self.label(value).to_owned())
                                    .size(12)
                                    .medium()
                                    .line_height(LineHeight::Absolute(16.into())),
                                remove,
                            ]
                            .spacing(0),
                        )
                        .padding(Padding::ZERO.left(6))
                        .height(21)
                        .style(|theme| theme::container::Style {
                            background: Some(theme.muted.base.into()),
                            text_color: Some(theme.neutral.foreground),
                            border: icedcn_core::Border {
                                radius: 4.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .into(),
                    );
                }
                chips.push(
                    input
                        .size(crate::Size::Xs)
                        .width(64)
                        .style(Box::new(control) as text_input::StyleFn<'a, Theme>)
                        .into(),
                );
                let mut content: Slot<'a, Event<Message>> = row::Row::with_children(chips)
                    .wrap()
                    .spacing(4)
                    .width(Fill)
                    .padding([4, 4])
                    .into();
                content.inherit(&inherited);
                Surface::new(
                    content.compose(),
                    if self.invalid {
                        Invalid.into()
                    } else {
                        text_input::Default.into()
                    },
                )
                .disabled(self.disabled)
                .boxed()
                .into()
            } else {
                let mut group = input_group(input);
                if self.invalid {
                    group = group.style(Invalid);
                }
                if self.popup {
                    group = group.style(Box::new(|theme: &Theme, status| {
                        let mut style = text_input::default(theme, status);
                        style.background = theme.input.alpha(0.3).into();
                        style.border.color = theme.input.alpha(0.3);
                        style
                    }) as text_input::StyleFn<'a, Theme>);
                }
                if let Some(leading) = &self.leading {
                    group = group.addon(input_group::addon(leading().map(Event::App)));
                }
                if !self.popup {
                    let end = if self.clear && !choice.is_empty() {
                        input_group::button(Icon::Remove.boxed())
                            .square()
                            .translate(false)
                            .on_press(Event::Clear)
                    } else {
                        input_group::button(chevron())
                            .square()
                            .translate(false)
                            .on_press(Event::Set(!open))
                    };
                    group =
                        group.addon(input_group::addon(end).align(input_group::Align::InlineEnd));
                }
                group.disabled(self.disabled).into()
            };
            field.inherit(&inherited);
            Keys {
                content: field.compose(),
                on_key: Box::new(move |key| match key {
                    Named::ArrowDown => Some(Event::Step(false)),
                    Named::ArrowUp => Some(Event::Step(true)),
                    Named::Enter if open => Some(Event::Submit),
                    Named::Home if open => Some(Event::Edge(false)),
                    Named::End if open => Some(Event::Edge(true)),
                    Named::Escape if open => Some(Event::Set(false)),
                    _ => None,
                }),
            }
            .boxed()
        };
        let field: Element<'a, Event<Message>, Theme, Renderer> = if self.popup {
            let label = if choice.is_empty() {
                self.placeholder.as_str()
            } else {
                choice
            };
            let mut trigger: Slot<'a, Event<Message>> = if let Some(render) = &self.trigger {
                let mut child = render(label);
                child.inherit(&inherited);
                marker::owner(
                    marker::marker(
                        child.compose().map(Event::App).boxed(),
                        Toggle,
                        Press::Captured,
                    )
                    .on_key(Named::ArrowDown, Toggle)
                    .boxed(),
                    move |Toggle| Event::Set(!open),
                )
                .boxed()
                .into()
            } else {
                button(row![text::small(label.to_owned()).width(Fill), chevron()].width(Fill))
                    .width(width)
                    .style(button::Outline)
                    .weight(Weight::Normal)
                    .disabled(self.disabled)
                    .on_press(Event::Set(!open))
                    .into()
            };
            trigger.inherit(&inherited);
            trigger.compose()
        } else {
            Slot::from(Container::new(make_input()).width(Fill)).compose()
        };
        let mut field = Behavior::field(
            field,
            state.focus_epoch,
            if self.multiple {
                self.selected(state).len()
            } else {
                0
            },
            width,
            (!self.popup).then_some(if self.leading.is_some() { 69.2 } else { 49.2 }),
        );
        if self.multiple {
            let values = self.selected(state).to_vec();
            field = field.chips(query.is_empty(), move |index| {
                Event::Choose(values[index].clone())
            });
        }
        let panel: Element<'a, Event<Message>, Theme, Renderer> = if open {
            let mut rows = Vec::new();
            let mut group = None;
            for (position, &index) in state.visible.iter().enumerate() {
                let item = &self.items[index];
                if item.group.as_deref() != group {
                    if group.is_some() && self.separators {
                        rows.push(menu::separator());
                    }
                    if let Some(name) = &item.group {
                        rows.push(
                            Container::new(
                                text::muted(name.clone())
                                    .size(12)
                                    .line_height(LineHeight::Absolute(16.into())),
                            )
                            .padding([6, 8])
                            .into(),
                        );
                    }
                    group = item.group.as_deref();
                }
                let chosen = if self.multiple {
                    self.selected(state).contains(&item.value)
                } else {
                    current.as_deref() == Some(item.value.as_str())
                };
                let mut label: Slot<'a, Event<Message>> = if let Some(render) = &self.format_item {
                    render(item).map(Event::App)
                } else if let Some(detail) = &item.description {
                    crate::column![
                        text::small(item.label.clone())
                            .medium()
                            .line_height(LineHeight::Absolute(19.25.into())),
                        text::muted(detail.clone())
                            .size(12)
                            .line_height(LineHeight::Absolute(18.into())),
                    ]
                    .spacing(0)
                    .into()
                } else {
                    text::small(item.label.clone())
                        .line_height(LineHeight::Absolute(20.into()))
                        .into()
                };
                label.inherit(&inherited);
                let check: Slot<'a, Event<Message>> = if chosen {
                    Icon::Check.boxed().into()
                } else {
                    Space::new().width(16).height(16).into()
                };
                let active = state.highlight == Some(position);
                let mut line: Slot<'a, Event<Message>> =
                    row![Container::new(label).width(Fill), check]
                        .spacing(8)
                        .width(Fill)
                        .into();
                line.inherit(&inherited.with_weight(Weight::Normal));
                let paint: widget::StyleFn<'a, Theme> = Box::new(move |theme: &Theme, status| {
                    let status = if active {
                        Status::Hovered
                    } else {
                        status.into()
                    };
                    let mut surface = item::style(theme, status, item::Variant::Default);
                    if matches!(status, Status::Hovered | Status::Pressed) {
                        surface.background = Some(theme.neutral.foreground.alpha(0.1).into());
                    }
                    surface.into()
                });
                let option = Button::new(line.compose())
                    .width(Fill)
                    .padding(Padding {
                        top: 4.0,
                        bottom: 4.0,
                        left: 6.0,
                        right: 8.0,
                    })
                    .translate(false)
                    .disabled(self.disabled)
                    .class(paint)
                    .on_press(Event::Choose(item.value.clone()));
                rows.push(
                    Container::new(option.boxed())
                        .width(Fill)
                        .id(state.ids[index].clone())
                        .into(),
                );
            }
            let empty = rows.is_empty();
            let body: Slot<'a, Event<Message>> = if empty {
                Container::new(
                    text::muted(self.empty.clone())
                        .size(14)
                        .line_height(LineHeight::Absolute(20.into())),
                )
                .center_x(Fill)
                .padding([8, 0])
                .into()
            } else {
                column::Column::with_children(rows).width(Fill).into()
            };
            let results =
                scrollable(
                    Container::new(body)
                        .width(Fill)
                        .padding(if empty { 0 } else { 4 }),
                )
                .id(state
                    .scroll
                    .clone()
                    .expect("combobox diff initializes its scroll id"))
                .width(Fill)
                .height(Length::Shrink.max(252));
            let mut body: Slot<'a, Event<Message>> = if self.popup {
                crate::column![
                    Container::new(make_input()).padding(Padding::ZERO.top(4).left(4).right(4)),
                    results
                ]
                .width(Fill)
                .into()
            } else {
                results.into()
            };
            body.inherit(&inherited);
            let content = Container::new(body)
                .width(self.content_width.map_or(Fill, Length::Fixed))
                .style(|theme| {
                    let mut surface = content(theme);
                    surface.background = Some(theme.popover.base.alpha(0.7).into());
                    surface
                });
            Behavior::panel(
                Slot::from(content).compose(),
                self.popup,
                state
                    .highlight
                    .and_then(|position| state.visible.get(position))
                    .map(|&index| {
                        (
                            state.scroll.clone().expect("scroll id"),
                            state.ids[index].clone(),
                        )
                    }),
                Event::Dismiss,
                Event::Set(false),
            )
            .boxed()
        } else {
            Space::new().boxed()
        };
        anchored(field, panel)
            .side(anchored::Side::Bottom)
            .align(anchored::Align::Start)
            .offset(6)
            .align_offset(self.align_offset)
            .width(if self.content_width.is_some() {
                anchored::Width::Content
            } else {
                anchored::Width::AtLeastAnchor
            })
            .open(open)
            .on_toggle(|_| Event::Dismiss)
    }
}

impl<'a, Message: Clone + 'static> Compose<Message> for Combobox<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
        self.disabled |= from.is_disabled();
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

/// Lucide globe for the Input Group example; no application icon font required.
pub fn globe<'a, Message: 'a>() -> Slot<'a, Message> {
    Icon::Globe.boxed().into()
}

#[derive(Clone, Copy)]
struct Toggle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values_seed_once_even_before_the_first_interaction() {
        let mut state = State::default();
        let mut first = combobox::<String>(["a", "b"]).default_value("a");
        Component::diff(&mut first, &mut state);
        let mut next = combobox::<String>(["a", "b"]).default_value("b");
        Component::diff(&mut next, &mut state);
        assert_eq!(next.current(&state), Some("a"));
        let mut state = State::default();
        let mut first = combobox::<Vec<String>>(["a", "b"]).default_values(["a"]);
        Component::diff(&mut first, &mut state);
        let mut next = combobox::<Vec<String>>(["a", "b"]).default_values(["b"]);
        Component::diff(&mut next, &mut state);
        assert_eq!(next.selected(&state), ["a"]);
    }

    #[test]
    fn controlled_single_rejection_does_not_mutate_selection() {
        let mut field = combobox(["a", "b"])
            .value("a")
            .on_value_change(|value| value);
        let mut state = State::default();
        Component::diff(&mut field, &mut state);
        assert_eq!(field.choose(&mut state, "b".into()), Some("b".into()));
        assert_eq!(field.current(&state), Some("a"));
    }
}
