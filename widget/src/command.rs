//! Filterable command lists with grouped actions, shortcut hints and keyboard selection.
use iced_core::Length::Fill;

mod behavior;

use crate::button::widget;
use crate::compose::{Compose, Inherited, Slot};
use crate::dialog::{Dialog, Handle};
use crate::icon::Icon;
use crate::text::Text;
use crate::{Input, column, container, input, input_group, menu, row, scrollable, separator, text};
use behavior::Behavior;
use iced_core::keyboard::{Key, key::Named};
use iced_core::widget::Id;
use iced_core::{Alignment, Element, Length, Padding, Pixels};
use iced_widget::component::{self, Component};
use iced_widget::text::StyleFn;
use iced_widget::{Renderer, Space, Widget, mouse_area};
use icedcn_core::theme::input_group::control;
use icedcn_core::theme::scrollable::hidden;
use icedcn_core::theme::text_input::Status;
use icedcn_core::{Border, Theme, theme};

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
    modal: bool,
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
        modal: false,
        inherited: Inherited::default(),
    }
}
/// Build a modal command palette, with an uncontrolled trigger and search focus.
/// The returned dialog also supports `open`, `default_open`, and `on_open_change`.
pub fn dialog<'a, Message: Clone + 'static>(
    trigger: impl Fn(&Handle) -> Slot<'a, Message> + 'a,
    content: impl Fn() -> Command<'a, Message> + 'a,
) -> Dialog<'a, Message> {
    crate::dialog(trigger, move |_| {
        let mut palette = content();
        palette.modal = true;
        palette.into()
    })
    .padding(0)
    .top(1.0 / 3.0)
    .max_width(512.0)
    .show_close_button(false)
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
/// Local query and keyboard selection, retained in the widget tree.
#[derive(Default)]
pub struct State {
    query: String,
    active: usize,
    scroll: Option<Id>,
    ids: Vec<Id>,
    keyboard: bool,
}
/// Internal events used by an embedded search field.
#[derive(Debug, Clone)]
pub enum Event {
    Input(String),
    Step(bool),
    Hover(usize),
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
    fn diff(&mut self, state: &mut State) {
        state.scroll.get_or_insert_with(Id::unique);
        state.ids.resize_with(self.items.len(), Id::unique);
        let count = self
            .filtered(&state.query)
            .filter(|(_, item)| !item.disabled)
            .count();
        state.active = state.active.min(count.saturating_sub(1));
    }
    fn update(&self, st: &mut State, event: Event, _: &Renderer) -> Option<Message> {
        if self.inherited.is_disabled() {
            return None;
        }
        let enabled: Vec<_> = self
            .filtered(&st.query)
            .filter(|(_, item)| !item.disabled)
            .map(|(i, _)| i)
            .collect();
        let n = enabled.len();
        st.keyboard = !matches!(event, Event::Hover(_) | Event::Choose(_));
        match event {
            Event::Input(q) => {
                st.query = q;
                st.active = 0;
            }
            Event::Step(back) if n > 0 => {
                st.active = if back {
                    st.active.saturating_sub(1)
                } else {
                    (st.active + 1).min(n - 1)
                }
            }
            Event::Hover(position) => st.active = position,
            Event::First => st.active = 0,
            Event::Last => st.active = n.saturating_sub(1),
            Event::Submit => return self.items.get(*enabled.get(st.active)?)?.on_select.clone(),
            Event::Choose(i) => {
                st.active = enabled.iter().position(|&index| index == i)?;
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
        let embedded = self.field.is_some();
        let mut group = None;
        let mut enabled_index = 0;
        let mut results = Vec::new();
        let mut rows = Vec::new();
        let mut selected = None;
        for (index, item) in self.filtered(&st.query) {
            if item.group.as_ref() != group {
                if !rows.is_empty() {
                    results.push(
                        column(std::mem::take(&mut rows))
                            .width(Fill)
                            .padding(4)
                            .into(),
                    );
                    results.push(separator().into());
                } else if embedded && group.is_some() {
                    results.push(menu::separator());
                }
                if let Some(title) = &item.group {
                    let heading = if embedded {
                        menu::label(title.clone())
                    } else {
                        container(
                            text::muted(title.clone())
                                .size(12)
                                .line_height(Pixels(16.))
                                .medium(),
                        )
                        .padding([6, 8])
                        .into()
                    };
                    if embedded {
                        results.push(heading);
                    } else {
                        rows.push(heading);
                    }
                }
                group = item.group.as_ref();
            }
            let label: Slot<'a, Event> = if let Some(icon) = &item.icon {
                row![
                    icon().size(16).line_height(Pixels(16.)),
                    Text::new(item.label.clone())
                        .size(14)
                        .line_height(Pixels(20.))
                ]
                .align_y(Alignment::Center)
                .spacing(8)
                .into()
            } else {
                Text::new(item.label.clone())
                    .size(14)
                    .line_height(Pixels(20.))
                    .into()
            };
            let active = !item.disabled && enabled_index == st.active;
            let position = enabled_index;
            if !item.disabled {
                enabled_index += 1;
            }
            if active {
                selected = Some(st.ids[index].clone());
            }
            if embedded {
                let mut entry = menu::item(label)
                    .disabled(item.disabled)
                    .highlighted(active)
                    .closes(false)
                    .on_select(Event::Choose(index));
                if let Some(hint) = item.shortcut {
                    entry = entry.shortcut(hint);
                }
                results.push(entry.into());
                continue;
            }
            let mut line = row![label, Space::new().width(Fill)]
                .width(Fill)
                .align_y(Alignment::Center);
            if let Some(hint) = item.shortcut {
                line = line.push(
                    text(hint)
                        .size(12)
                        .line_height(Pixels(16.))
                        .letter_spacing(0.1)
                        .style(Box::new(move |theme: &Theme| {
                            theme::text::Style {
                                color: Some(if active {
                                    theme.popover.foreground
                                } else {
                                    theme.muted.foreground
                                }),
                            }
                            .into()
                        }) as StyleFn<'_, Theme>)
                        .into(),
                );
            }
            let modal = self.modal;
            let disabled = item.disabled || self.inherited.is_disabled();
            let mut line: Slot<'a, Event> = line.into();
            line.inherit(&self.inherited.with_line_height(20.).with_selectable(false));
            let entry = widget::Button::new(line.compose())
                .width(Fill)
                .padding([6, 8])
                .translate(false)
                .disabled(disabled)
                .on_press(Event::Choose(index))
                .style(Box::new(move |theme: &Theme, _| widget::Style {
                    background: active.then(|| theme.muted.base.into()),
                    text_color: if disabled {
                        theme.popover.foreground.scale_alpha(0.5).into()
                    } else {
                        theme.popover.foreground.into()
                    },
                    border: iced_core::Border {
                        radius: if modal { 10. } else { 6. }.into(),
                        ..Default::default()
                    },
                    ..widget::Style::default()
                }) as widget::StyleFn<'_, Theme>);
            let entry = mouse_area(entry);
            let entry = if !active && !disabled {
                entry.on_move(move |_| Event::Hover(position))
            } else {
                entry
            };
            rows.push(
                container(entry.boxed())
                    .width(Fill)
                    .id(st.ids[index].clone())
                    .into(),
            );
        }
        if !rows.is_empty() {
            results.push(column(rows).width(Fill).padding(4).into());
        }
        if results.is_empty() {
            results.push(
                container(text(self.empty.clone()).size(14).line_height(Pixels(20.)))
                    .center_x(Fill)
                    .padding([24, 0])
                    .into(),
            );
        }
        let search = input(self.placeholder.clone())
            .value(st.query.clone())
            .on_input(Event::Input)
            .width(Fill);
        let search = match &self.field {
            Some(build) => build(search),
            None => {
                let icon = container(Icon::Search.boxed()).style(|theme: &Theme| {
                    theme::container::Style {
                        text_color: Some(theme.muted.foreground.scale_alpha(0.5)),
                        ..Default::default()
                    }
                });
                let field = input_group(search)
                    .addon(input_group::addon(icon))
                    .style(Box::new(|theme: &Theme, _| {
                        let mut style = control(theme, Status::Active);
                        style.background = theme.input.scale_alpha(0.3).into();
                        style.border = Border::default()
                            .width(1)
                            .rounded(10)
                            .color(theme.input.scale_alpha(0.3));
                        style.ring = None;
                        style
                    })
                        as theme::text_input::StyleFn<'_, Theme>);
                container(field)
                    .width(Fill)
                    .padding(Padding::ZERO.top(4).left(4).right(4))
                    .into()
            }
        };
        let list = scrollable(column(results).width(Fill))
            .id(st
                .scroll
                .clone()
                .expect("command diff initializes scroll id"))
            .width(Fill)
            .height(Length::Shrink.max(if embedded { 300 } else { 288 }));
        let list = if embedded { list } else { list.style(hidden) };
        let modal = self.modal;
        let mut palette: Slot<'a, Event> = if embedded {
            column![search, list].width(self.width).spacing(8).into()
        } else {
            container(column![search, list].width(Fill))
                .padding(if modal { 4 } else { 5 })
                .width(self.width)
                .style(move |theme: &Theme| theme::container::Style {
                    background: Some(theme.popover.base.into()),
                    text_color: Some(theme.popover.foreground),
                    border: Border::default()
                        .rounded(14)
                        .width(if modal { 0 } else { 1 })
                        .color(theme.neutral.border),
                    ..Default::default()
                })
                .into()
        };
        palette.inherit(&self.inherited);
        Behavior {
            is_hover: |event| matches!(event, Event::Hover(_)),
            content: palette.compose(),
            reveal: if st.keyboard && !embedded {
                selected.map(|id| (st.scroll.clone().expect("scroll id"), id))
            } else {
                None
            },
            on_key: Box::new(|key| match key {
                Key::Named(Named::ArrowUp) => Some(Event::Step(true)),
                Key::Named(Named::ArrowDown) => Some(Event::Step(false)),
                Key::Named(Named::Home) => Some(Event::First),
                Key::Named(Named::End) => Some(Event::Last),
                Key::Named(Named::Enter) => Some(Event::Submit),
                _ => None,
            }),
        }
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
