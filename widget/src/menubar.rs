//! A persistent row of menus sharing a single open panel.
use std::borrow::Cow;
use std::rc::Rc;
use std::time::Duration;

use crate::button::widget;
use crate::button::widget::Button;
use crate::combobox::keys::Keys;
use crate::compose::{Compose, Inherited, Slot};
use crate::menu::list::List;
use crate::{container, popover, row, text};
use iced_core::keyboard::{Key, key::Named};
use iced_core::{Alignment, Element, Length, Pixels};
use iced_widget::button::Status;
use iced_widget::{
    Renderer, Widget,
    component::{self, Component},
    mouse_area,
};
use icedcn_core::theme::button::ghost;
use icedcn_core::theme::container::Style;
use icedcn_core::{Theme, radius};

/// A menu trigger and the content displayed below it, or a navigation link.
#[derive(Clone)]
pub struct Entry<'a, Message> {
    label: Cow<'a, str>,
    action: Action<'a, Message>,
    width: f32,
}
#[derive(Clone)]
enum Action<'a, Message> {
    Menu(Rc<dyn Fn() -> Slot<'a, Message> + 'a>),
    Link(Message),
}

/// A named menu containing ordinary menu items, groups and submenus.
pub fn menu<'a, Message>(
    label: impl Into<Cow<'a, str>>,
    content: impl Fn() -> Slot<'a, Message> + 'a,
) -> Entry<'a, Message> {
    Entry {
        label: label.into(),
        action: Action::Menu(Rc::new(content)),
        width: 176.,
    }
}

pub(crate) fn link<'a, Message>(
    label: impl Into<Cow<'a, str>>,
    message: Message,
) -> Entry<'a, Message> {
    Entry {
        label: label.into(),
        action: Action::Link(message),
        width: 0.,
    }
}
impl<Message> Entry<'_, Message> {
    /// Set the width of this menu's content in logical pixels.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into().0;
        self
    }
}

/// The bar, which can be controlled with a zero-based menu index.
pub struct Menubar<'a, Message> {
    entries: Vec<Entry<'a, Message>>,
    width: Length,
    navigation: bool,
    value: Option<Option<usize>>,
    on_change: Option<Box<dyn Fn(Option<usize>) -> Message + 'a>>,
    inherited: Inherited,
}

/// Create a menubar with keyboard navigation between its entries.
pub fn menubar<'a, Message>(
    entries: impl IntoIterator<Item = Entry<'a, Message>>,
) -> Menubar<'a, Message> {
    Menubar {
        entries: entries.into_iter().collect(),
        width: Length::Shrink,
        navigation: false,
        value: None,
        on_change: None,
        inherited: Inherited::default(),
    }
}
impl<'a, Message> Menubar<'a, Message> {
    pub(crate) fn navigation(mut self) -> Self {
        self.navigation = true;
        self
    }
    /// Set the width of the outer bar.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
    /// Control which menu is expanded.
    pub fn value(mut self, value: Option<usize>) -> Self {
        self.value = Some(value);
        self
    }
    /// Receive changes to the expanded entry.
    pub fn on_value_change(mut self, change: impl Fn(Option<usize>) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(change));
        self
    }
}
#[derive(Default)]
pub struct State {
    active: Option<usize>,
    keyboard: bool,
}
#[derive(Clone)]
pub enum Event<Message> {
    Show(usize, bool),
    Hide(usize),
    Move(usize),
    App(Message),
}

impl<'a, Message: Clone + 'static> Component<'a, Message, Theme, Renderer>
    for Menubar<'a, Message>
{
    type State = State;
    type Event = Event<Message>;
    fn update(&self, state: &mut State, event: Event<Message>, _: &Renderer) -> Option<Message> {
        let active = self.value.unwrap_or(state.active);
        let next = match event {
            Event::App(message) => return Some(message),
            Event::Hide(i) if active == Some(i) => None,
            Event::Move(i) if active.is_some() => self
                .entries
                .get(i)
                .filter(|e| matches!(e.action, Action::Menu(_)))
                .map(|_| i),
            Event::Show(i, keyboard) => {
                state.keyboard = keyboard;
                Some(i)
            }
            _ => return None,
        };
        if active == next {
            return None;
        }
        if self.value.is_none() {
            state.active = next;
        }
        self.on_change.as_ref().map(|change| change(next))
    }
    fn view(&self, state: &State) -> impl Widget<Event<Message>, Theme, Renderer> + 'a {
        let active = self.value.unwrap_or(state.active);
        let navigation = self.navigation;
        let keyboard = state.keyboard;
        let mut items = row::Row::new()
            .align_y(Alignment::Center)
            .spacing(if navigation { 0 } else { 2 });
        for (i, entry) in self.entries.iter().cloned().enumerate() {
            let is_open = active == Some(i);
            let expandable = matches!(entry.action, Action::Menu(_));
            let trigger = move || {
                let label = text(entry.label.clone()).size(text::size::SM).medium();
                let content: Slot<'a, Event<Message>> = if navigation && expandable {
                    row![label, text(if is_open { "⌃" } else { "⌄" }).size(12)]
                        .spacing(4)
                        .align_y(Alignment::Center)
                        .into()
                } else {
                    label.into()
                };
                let style: widget::StyleFn<'a, Theme> =
                    Box::new(move |theme: &Theme, status: widget::Status| {
                        let style: iced_widget::button::Style = ghost(
                            theme,
                            if is_open {
                                Status::Hovered
                            } else {
                                status.into()
                            },
                        )
                        .into();
                        style.into()
                    });
                Button::new(content.compose())
                    .height(if navigation { 36 } else { 24 })
                    .padding(if navigation { [6, 10] } else { [2, 6] })
                    .translate(false)
                    .style(style)
            };
            let entry: Slot<'a, Event<Message>> = match entry.action {
                Action::Link(ref message) => trigger()
                    .on_press(Event::App(message.clone()))
                    .boxed()
                    .into(),
                Action::Menu(ref panel) => {
                    let panel = Rc::clone(panel);
                    let inherited = self.inherited;
                    let mut pop = popover(
                        move |p| {
                            let mut trigger = trigger().boxed();
                            if active.is_some() && !navigation {
                                trigger =
                                    mouse_area(trigger).on_enter(Event::Show(i, false)).boxed();
                            }
                            p.trigger(
                                Keys {
                                    content: trigger,
                                    on_key: Box::new(move |key| {
                                        (key == Key::Named(Named::ArrowDown))
                                            .then_some(Event::Show(i, true))
                                    }),
                                }
                                .boxed(),
                            )
                        },
                        move |_| {
                            let mut body = panel();
                            body.inherit(&inherited);
                            List {
                                content: body.map(Event::App).compose(),
                                horizontal: Some(false),
                                autofocus: keyboard.into(),
                                on_move: None,
                            }
                            .boxed()
                            .into()
                        },
                    )
                    .open(is_open)
                    .on_open_change(move |open| {
                        if open {
                            Event::Show(i, false)
                        } else {
                            Event::Hide(i)
                        }
                    })
                    .align(popover::Align::Start)
                    .side_offset(if navigation { 6 } else { 8 })
                    .align_offset(if navigation { 0.0 } else { -4.0 })
                    .width(entry.width)
                    .padding(4);
                    if navigation {
                        pop = pop.hover(Duration::from_millis(180), Duration::from_millis(150));
                    }
                    pop.into()
                }
            };
            items = items.push(entry);
        }
        let mut frame = container(items).width(self.width);
        if !navigation {
            frame = frame.padding(3).height(32).style(|theme: &Theme| {
                let mut style = Style::default();
                style.border.width = 1.;
                style.border.radius = radius::LG.into();
                style.border.color = theme.neutral.border;
                style
            });
        }
        List {
            content: Slot::from(frame).compose(),
            horizontal: Some(true),
            autofocus: false.into(),
            on_move: Some(Box::new(Event::Move)),
        }
    }
}
impl<'a, Message: Clone + 'static> Compose<Message> for Menubar<'a, Message> {
    fn inherit(&mut self, inherited: &Inherited) {
        self.inherited = self.inherited.or(*inherited);
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}
impl<'a, Message: Clone + 'static> From<Menubar<'a, Message>> for Slot<'a, Message> {
    fn from(bar: Menubar<'a, Message>) -> Self {
        Slot::new(bar)
    }
}
impl<'a, Message: Clone + 'static> From<Menubar<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(bar: Menubar<'a, Message>) -> Self {
        Slot::from(bar).compose()
    }
}
