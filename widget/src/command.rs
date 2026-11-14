//! Filterable command lists with grouped actions, shortcut hints and keyboard selection.
use iced_core::Length::Fill;

use crate::combobox::keys::Keys;
use crate::compose::{Compose, Inherited, Slot};
use crate::{Input, column, container, input, menu, row, scrollable, text};
use iced_core::keyboard::key::Named;
use iced_core::{Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedxdy_core::{Theme, theme::popover::content};

/// An action in a command list.
pub struct Item<'a, Message> {
    label: String,
    search: String,
    group: Option<String>,
    shortcut: Option<&'a str>,
    disabled: bool,
    icon: Option<Box<dyn Fn() -> iced_widget::Text<'a, Theme> + 'a>>,
    on_select: Option<Message>,
}
/// Build a command option, identified and filtered by its label.
pub fn item<'a, Message>(label: impl Into<String>) -> Item<'a, Message> {
    let label = label.into();
    Item {
        search: label.to_lowercase(),
        label,
        group: None,
        shortcut: None,
        disabled: false,
        icon: None,
        on_select: None,
    }
}
impl<'a, Message> Item<'a, Message> {
    /// Heading above consecutive options with this group.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }
    /// Keyboard hint displayed at the end of this row.
    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }
    /// Prevent selection while preserving the visible result.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Place an icon before the command label.
    pub fn icon(mut self, icon: impl Fn() -> iced_widget::Text<'a, Theme> + 'a) -> Self {
        self.icon = Some(Box::new(icon));
        self
    }
    /// Publish an application action when the command is chosen.
    pub fn on_select(mut self, message: Message) -> Self {
        self.on_select = Some(message);
        self
    }
}
/// An inline command search. Place it in a Dialog for modal presentation.
pub struct Command<'a, Message> {
    items: Vec<Item<'a, Message>>,
    placeholder: String,
    empty: String,
    width: Length,
    limit: usize,
    field: Option<Field<'a>>,
    inherited: Inherited,
}
type Field<'a> = Box<dyn Fn(Input<'a, Event>) -> Slot<'a, Event> + 'a>;

/// Build a list from its source order. Filtering retains all matching groups.
pub fn command<'a, Message>(
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> Command<'a, Message> {
    Command {
        items: items.into_iter().collect(),
        placeholder: "Type a command or search...".into(),
        empty: "No results found.".into(),
        width: Fill,
        limit: usize::MAX,
        field: None,
        inherited: Inherited::default(),
    }
}
impl<'a, Message> Command<'a, Message> {
    /// Input placeholder.
    pub fn placeholder(mut self, label: impl Into<String>) -> Self {
        self.placeholder = label.into();
        self
    }
    /// Empty result message.
    pub fn empty(mut self, label: impl Into<String>) -> Self {
        self.empty = label.into();
        self
    }
    /// Width of the palette.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
    /// Cap the visible results after filtering a large command list.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Embed the results in a surface supplied by the caller. The field
    /// receives the current query and already publishes edits to the list.
    pub fn embedded(mut self, field: impl Fn(Input<'a, Event>) -> Slot<'a, Event> + 'a) -> Self {
        self.field = Some(Box::new(field));
        self
    }

    fn filtered(&self, query: &str) -> impl Iterator<Item = (usize, &Item<'a, Message>)> {
        let query = query.to_lowercase();
        self.items
            .iter()
            .enumerate()
            .filter(move |(_, item)| {
                query
                    .split_whitespace()
                    .all(|word| item.search.contains(word))
            })
            .take(self.limit)
    }
}
#[derive(Default)]
pub struct State {
    query: String,
    active: usize,
}
#[derive(Debug, Clone)]
pub enum Event {
    Input(String),
    Step(bool),
    First,
    Last,
    Choose(usize),
    Submit,
}
impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer>
    for Command<'a, Message>
{
    type State = State;
    type Event = Event;
    fn update(&self, st: &mut State, event: Event, _: &Renderer) -> Option<Message> {
        let enabled: Vec<_> = self
            .filtered(&st.query)
            .filter(|(_, item)| !item.disabled)
            .map(|(i, _)| i)
            .collect();
        let n = enabled.len();
        match event {
            Event::Input(q) => {
                st.query = q;
                st.active = 0;
            }
            Event::Step(back) if n > 0 => {
                st.active = if back {
                    (st.active + n - 1) % n
                } else {
                    (st.active + 1) % n
                }
            }
            Event::First => st.active = 0,
            Event::Last => st.active = n.saturating_sub(1),
            Event::Submit => return self.items.get(*enabled.get(st.active)?)?.on_select.clone(),
            Event::Choose(i) => {
                return self
                    .items
                    .get(i)
                    .filter(|item| !item.disabled)?
                    .on_select
                    .clone();
            }
            Event::Step(_) => {}
        }
        None
    }
    fn view(&self, st: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        let mut group = None;
        let mut enabled_index = 0;
        let mut results = Vec::new();
        for (index, item) in self.filtered(&st.query) {
            if item.group.as_ref() != group {
                if group.is_some() {
                    results.push(menu::separator());
                }
                if let Some(title) = &item.group {
                    results.push(menu::label(title.clone()));
                }
                group = item.group.as_ref();
            }
            let label: Slot<'a, Event> = if let Some(icon) = &item.icon {
                row![icon().size(16), item.label.clone()].spacing(8).into()
            } else {
                item.label.clone().into()
            };
            let active = !item.disabled && enabled_index == st.active;
            if !item.disabled {
                enabled_index += 1;
            }
            let mut entry = menu::item(label)
                .disabled(item.disabled)
                .highlighted(active)
                .closes(false)
                .on_select(Event::Choose(index));
            if let Some(hint) = item.shortcut {
                entry = entry.shortcut(hint);
            }
            results.push(entry.into());
        }
        if results.is_empty() {
            results.push(
                container(text::muted(self.empty.clone()))
                    .center_x(Fill)
                    .padding(16)
                    .into(),
            );
        }
        let search = input(self.placeholder.clone())
            .value(st.query.clone())
            .on_input(Event::Input)
            .width(Fill);
        let search = match &self.field {
            Some(build) => build(search),
            None => search.into(),
        };
        let search = Keys {
            content: search.compose(),
            on_key: Box::new(|key| match key {
                Named::ArrowUp => Some(Event::Step(true)),
                Named::ArrowDown => Some(Event::Step(false)),
                Named::Home => Some(Event::First),
                Named::End => Some(Event::Last),
                Named::Enter => Some(Event::Submit),
                _ => None,
            }),
        };
        let embedded = self.field.is_some();
        let list = scrollable(
            column(results)
                .width(Fill)
                .padding(if embedded { 0 } else { 4 }),
        )
        .width(Fill)
        .height(Length::Shrink.max(300));
        let mut palette: Slot<'a, Event> = if embedded {
            column![search.boxed(), list]
                .width(self.width)
                .spacing(8)
                .into()
        } else {
            container(column![search.boxed(), menu::separator(), list].width(Fill))
                .padding(4)
                .width(self.width)
                .style(content)
                .into()
        };
        palette.inherit(&self.inherited);
        palette.compose()
    }
}
impl<'a, Message: Clone + 'static> Compose<Message> for Command<'a, Message> {
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
impl<'a, Message: Clone + 'static> From<Command<'a, Message>> for Slot<'a, Message> {
    fn from(value: Command<'a, Message>) -> Self {
        Slot::new(value)
    }
}
impl<'a, Message: Clone + 'static> From<Command<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(value: Command<'a, Message>) -> Self {
        component::component(value).boxed()
    }
}
