//! A Calendar in an anchored Popover with a controlled or uncontrolled selection.

use iced_core::Element;
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;
use std::rc::Rc;

use crate::calendar::{Mode, Selection};
use crate::compose::{Compose, Inherited, Slot};
use crate::{button, calendar, popover};

type Label<'a, Message> = Rc<dyn Fn(Option<Selection>) -> Slot<'a, Message> + 'a>;

pub struct DatePicker<'a, Message> {
    mode: Mode,
    value: Option<Option<Selection>>,
    initial: Option<Selection>,
    placeholder: String,
    label: Option<Label<'a, Event>>,
    caption: bool,
    disabled: bool,
    width: f32,
    on_change: Option<Box<dyn Fn(Option<Selection>) -> Message + 'a>>,
}

pub fn date_picker<'a, Message>() -> DatePicker<'a, Message> {
    DatePicker {
        mode: Mode::Single,
        value: None,
        initial: None,
        placeholder: "Pick a date".to_owned(),
        label: None,
        caption: false,
        disabled: false,
        width: 208.,
        on_change: None,
    }
}

impl<'a, Message> DatePicker<'a, Message> {
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
    pub fn value(mut self, value: Option<Selection>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn default_value(mut self, value: Selection) -> Self {
        self.initial = Some(value);
        self
    }
    pub fn on_value_change(mut self, f: impl Fn(Option<Selection>) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    pub fn label(mut self, f: impl Fn(Option<Selection>) -> Slot<'a, Event> + 'a) -> Self {
        self.label = Some(Rc::new(f));
        self
    }
    pub fn caption_dropdown(mut self, enabled: bool) -> Self {
        self.caption = enabled;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn width(mut self, width: impl Into<iced_core::Pixels>) -> Self {
        self.width = width.into().0.max(1.);
        self
    }
}

#[derive(Default)]
pub struct State {
    selection: Option<Option<Selection>>,
    open: bool,
}
#[derive(Debug, Clone)]
pub enum Event {
    Open(bool),
    Pick(Option<Selection>),
}

impl<Message> DatePicker<'_, Message> {
    fn apply(&self, state: &mut State, event: Event) -> Option<Message> {
        if self.disabled {
            return None;
        }
        match event {
            Event::Open(open) => {
                state.open = open;
                None
            }
            Event::Pick(selection) => {
                if self.value.is_none() {
                    state.selection = Some(selection);
                }
                if self.mode == Mode::Single || selection.is_some_and(|s| s.end.is_some()) {
                    state.open = false;
                }
                self.on_change.as_ref().map(|f| f(selection))
            }
        }
    }
}
impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for DatePicker<'a, Message> {
    type State = State;
    type Event = Event;
    fn update(&self, state: &mut State, event: Event, _: &Renderer) -> Option<Message> {
        self.apply(state, event)
    }
    fn view(&self, state: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        let selected = self.value.or(state.selection).unwrap_or(self.initial);
        let label = self.label.clone();
        let caption = self.caption;
        let disabled = self.disabled;
        let mode = self.mode;
        let width = self.width;
        let default_text = selected.map_or_else(
            || self.placeholder.clone(),
            |s| match s.end {
                Some(end) => format!(
                    "{} – {}",
                    s.start.format("%b %d, %Y"),
                    end.format("%b %d, %Y")
                ),
                None => s.start.format("%B %-d, %Y").to_string(),
            },
        );
        let entry =
            popover(
                move |p| {
                    p.trigger(
                        button(label.as_ref().map_or_else(
                            || Slot::from(default_text.clone()),
                            |build| build(selected),
                        ))
                        .style(button::Outline)
                        .width(width)
                        .disabled(disabled),
                    )
                },
                move |_| {
                    calendar()
                        .mode(mode)
                        .selected(selected)
                        .caption_dropdown(caption)
                        .number_of_months(if mode == Mode::Range { 2 } else { 1 })
                        .on_select(Event::Pick)
                        .into()
                },
            )
            .align(popover::Align::Start)
            .width(if mode == Mode::Range { 508. } else { 256. })
            .open(state.open && !disabled)
            .on_open_change(Event::Open);
        Element::from(entry)
    }
}
impl<'a, Message: 'static> Compose<Message> for DatePicker<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}
impl<'a, Message: 'static> From<DatePicker<'a, Message>> for Slot<'a, Message> {
    fn from(p: DatePicker<'a, Message>) -> Self {
        Self::new(p)
    }
}
impl<'a, Message: 'static> From<DatePicker<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(p: DatePicker<'a, Message>) -> Self {
        Box::new(p).compose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::Date;

    #[test]
    fn incomplete_range_stays_open_until_its_second_day_is_picked() {
        let picker = date_picker::<()>().mode(Mode::Range);
        let mut state = State {
            open: true,
            ..State::default()
        };
        let start = Date::from_ymd_opt(2026, 3, 8).unwrap();
        let end = Date::from_ymd_opt(2026, 3, 12).unwrap();
        picker.apply(&mut state, Event::Pick(Some(Selection::single(start))));
        assert!(state.open);
        picker.apply(&mut state, Event::Pick(Some(Selection::range(start, end))));
        assert!(!state.open);
        assert_eq!(state.selection, Some(Some(Selection::range(start, end))));
    }
}
