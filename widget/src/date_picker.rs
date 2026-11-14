//! A Calendar in an anchored Popover with a controlled or uncontrolled selection.

mod input;
pub use input::{Input, input, relative};

use crate::icon::Icon;
use crate::{Weight, row, space, text};
use iced_core::Length::Fill;
use iced_core::{Alignment, Element, Length};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;
use icedcn_core::theme;
use std::fmt::Write as _;
use std::rc::Rc;

use crate::calendar::{Datelike, Mode, Selection};
use crate::compose::{Compose, Inherited, Slot};
use crate::{button, calendar, popover};

type Label<'a, Message> = Rc<dyn Fn(Option<Selection>) -> Slot<'a, Message> + 'a>;

pub struct DatePicker<'a, Message> {
    mode: Mode,
    value: Option<Option<Selection>>,
    initial: Option<Selection>,
    placeholder: String,
    format: Option<String>,
    label: Option<Label<'a, Event>>,
    initial_month: Option<calendar::Date>,
    caption: bool,
    disabled: bool,
    width: Length,
    chevron: bool,
    close_on_select: bool,
    stacked: Option<bool>,
    on_change: Option<Box<dyn Fn(Option<Selection>) -> Message + 'a>>,
}

pub fn date_picker<'a, Message>() -> DatePicker<'a, Message> {
    DatePicker {
        mode: Mode::Single,
        value: None,
        initial: None,
        placeholder: "Pick a date".to_owned(),
        format: None,
        label: None,
        initial_month: None,
        caption: false,
        disabled: false,
        width: Length::Fixed(212.),
        chevron: true,
        close_on_select: true,
        stacked: None,
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
    /// Format a single day using a strftime pattern.
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
    pub fn label(mut self, f: impl Fn(Option<Selection>) -> Slot<'a, Event> + 'a) -> Self {
        self.label = Some(Rc::new(f));
        self
    }
    /// Month to display when there is no selection.
    pub fn default_month(mut self, month: calendar::Date) -> Self {
        self.initial_month = Some(month);
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
    /// Show the trailing chevron on the standard single-date trigger.
    pub fn chevron(mut self, enabled: bool) -> Self {
        self.chevron = enabled;
        self
    }
    /// Close the single-date picker after choosing a day. Range selections stay open for editing.
    pub fn close_on_select(mut self, close: bool) -> Self {
        self.close_on_select = close;
        self
    }
    /// Arrange range months vertically at the application's compact window breakpoint.
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = Some(stacked);
        self
    }
    /// Set the width of the trigger, independently of the open calendar.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
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
                if self.mode == Mode::Single && self.close_on_select {
                    state.open = false;
                }
                self.on_change.as_ref().map(|f| f(selection))
            }
        }
    }
}
fn long_date(date: calendar::Date) -> String {
    let day = date.day();
    let suffix = if (11..=13).contains(&(day % 100)) {
        "th"
    } else {
        match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };
    format!("{} {day}{suffix}, {}", date.format("%B"), date.year())
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
        let initial_month = selected.map(|s| s.start).or(self.initial_month);
        let caption = self.caption;
        let disabled = self.disabled;
        let mode = self.mode;
        let width = self.width;
        let chevron = self.chevron;
        let stacked = self.stacked;
        let default_text = selected.map_or_else(
            || self.placeholder.clone(),
            |s| match (mode, s.end) {
                (Mode::Range, Some(end)) => format!(
                    "{} - {}",
                    s.start.format("%b %d, %Y"),
                    end.format("%b %d, %Y")
                ),
                (Mode::Range, None) => s.start.format("%b %d, %Y").to_string(),
                _ => self.format.as_ref().map_or_else(
                    || long_date(s.start),
                    |f| {
                        let mut formatted = String::new();
                        if write!(formatted, "{}", s.start.format(f)).is_ok() {
                            formatted
                        } else {
                            long_date(s.start)
                        }
                    },
                ),
            },
        );
        let entry = popover(
            move |p| {
                let title = if let Some(custom) = &label {
                    custom(selected)
                } else {
                    let title = text::Text::new(default_text.clone());
                    let mut line = row::Row::new()
                        .width(Fill)
                        .align_y(Alignment::Center)
                        .spacing(6);
                    if mode == Mode::Range {
                        line = line.push(Icon::Calendar.boxed().into());
                    }
                    let faded = selected.is_none() && chevron && mode == Mode::Single;
                    line = line
                        .push(Slot::from(if faded {
                            title.style(theme::text::Muted)
                        } else {
                            title
                        }))
                        .push(space::horizontal().into());
                    if chevron && mode == Mode::Single {
                        line = line.push(Icon::Chevron.boxed().into());
                    }
                    line.into()
                };
                p.trigger_with_control(
                    button(title)
                        .weight(Weight::Normal)
                        .translate(false)
                        .style(button::Outline)
                        .width(width)
                        .disabled(disabled),
                    !disabled,
                )
            },
            move |_| {
                let mut calendar = calendar()
                    .weekday_line_height(18.285715)
                    .mode(mode)
                    .selected(selected)
                    .caption_dropdown(caption)
                    .number_of_months(if mode == Mode::Range { 2 } else { 1 })
                    .on_select(Event::Pick);
                if let Some(month) = initial_month {
                    calendar = calendar.default_month(month);
                }
                if let Some(stacked) = stacked {
                    calendar = calendar.stacked(stacked);
                }
                calendar.into()
            },
        )
        .align(popover::Align::Start)
        .padding(0)
        .focus_content(false)
        .width(if mode == Mode::Range && stacked != Some(true) {
            424.
        } else {
            212.
        })
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
    fn range_stays_open_for_endpoint_edits_until_dismissed() {
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
        assert!(state.open);
        assert_eq!(state.selection, Some(Some(Selection::range(start, end))));
        picker.apply(&mut state, Event::Open(false));
        assert!(!state.open);
    }

    #[test]
    fn single_selection_closes_the_picker() {
        let picker = date_picker::<()>();
        let mut state = State {
            open: true,
            ..State::default()
        };
        let day = Date::from_ymd_opt(2026, 3, 8).unwrap();
        picker.apply(&mut state, Event::Pick(Some(Selection::single(day))));
        assert!(!state.open);
        assert_eq!(state.selection, Some(Some(Selection::single(day))));
    }
}
