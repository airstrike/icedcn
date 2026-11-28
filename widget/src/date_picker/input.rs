use crate::calendar::{Date, Datelike, Duration, Selection, Weekday, today};
use crate::combobox::keys::Keys;
use crate::compose::{Compose, Inherited, View};
use crate::icon::Icon;
use crate::{calendar, input_group, popover};
use iced_core::{
    Element,
    keyboard::{Key, key::Named},
};
use iced_widget::{
    Renderer, Widget,
    component::{self, Component},
};
use icedcn_core::Theme;

/// An editable date paired with an anchored month calendar.
pub struct Input<'a, Message> {
    placeholder: String,
    initial_text: Option<String>,
    initial_date: Option<Date>,
    value: Option<Option<Date>>,
    parse: fn(&str, Date) -> Option<Date>,
    change: Option<Box<dyn Fn(Option<Date>) -> Message + 'a>>,
    caption: bool,
    disabled: bool,
    text_size: f32,
}

/// Enter a date by typing or choosing a calendar day.
pub fn input<'a, Message>(placeholder: impl Into<String>) -> Input<'a, Message> {
    Input {
        placeholder: placeholder.into(),
        initial_text: None,
        initial_date: None,
        value: None,
        parse: absolute,
        change: None,
        caption: false,
        disabled: false,
        text_size: 14.,
    }
}
impl<'a, Message> Input<'a, Message> {
    /// Seed the unedited input from this calendar date.
    pub fn default_value(mut self, date: Date) -> Self {
        self.initial_date = Some(date);
        self
    }
    /// Show a phrase as the initial text and parse it relative to today.
    pub fn default_text(mut self, text: impl Into<String>) -> Self {
        self.initial_text = Some(text.into());
        self
    }
    /// Control the selected day. In-progress text remains editable.
    pub fn value(mut self, date: Option<Date>) -> Self {
        self.value = Some(date);
        self
    }
    /// Replace the default absolute date parser.
    pub fn parse_with(mut self, parse: fn(&str, Date) -> Option<Date>) -> Self {
        self.parse = parse;
        self
    }
    /// Receive the selected day after a successful parse or a calendar selection.
    pub fn on_value_change(mut self, change: impl Fn(Option<Date>) -> Message + 'a) -> Self {
        self.change = Some(Box::new(change));
        self
    }
    /// Use month and year dropdowns in the calendar header.
    pub fn caption_dropdown(mut self, enabled: bool) -> Self {
        self.caption = enabled;
        self
    }
    /// Set the editing font size in logical pixels.
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Default)]
pub struct State {
    open: bool,
    keyboard: bool,
    text: String,
    date: Option<Date>,
    initialized: bool,
    controlled: Option<Option<Date>>,
}
#[derive(Debug, Clone)]
pub enum Event {
    Open(bool),
    OpenFromInput,
    Edit(String),
    Pick(Option<Selection>),
}
fn formatted(date: Option<Date>) -> String {
    date.map(|d| d.format("%B %d, %Y").to_string())
        .unwrap_or_default()
}
impl<'a, M: 'static> Component<'a, M, Theme, Renderer> for Input<'a, M> {
    type State = State;
    type Event = Event;
    fn diff(&mut self, state: &mut State) {
        if !state.initialized {
            state.date = self.value.unwrap_or_else(|| {
                self.initial_date.or_else(|| {
                    self.initial_text
                        .as_deref()
                        .and_then(|t| (self.parse)(t, today()))
                })
            });
            state.text = self
                .initial_text
                .clone()
                .unwrap_or_else(|| formatted(state.date));
            state.initialized = true;
        } else if self.value != state.controlled
            && let Some(date) = self.value
            && date != state.date
        {
            state.date = date;
            state.text = formatted(date);
        }
        state.controlled = self.value;
    }
    fn update(&self, state: &mut State, event: Event, _: &Renderer) -> Option<M> {
        if self.disabled {
            return None;
        }
        let selected = match event {
            Event::OpenFromInput => {
                state.open = true;
                state.keyboard = true;
                return None;
            }
            Event::Open(open) => {
                state.keyboard = false;
                state.open = open;
                return None;
            }
            Event::Edit(text) => {
                state.text = text;
                Some((self.parse)(&state.text, today())?)
            }
            Event::Pick(pick) => {
                let date = pick.map(|s| s.start);
                state.text = formatted(date);
                state.open = false;
                date
            }
        };
        state.date = selected;
        self.change.as_ref().map(|f| f(selected))
    }
    fn view(&self, state: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        let selected = self.value.unwrap_or(state.date);
        let caption = self.caption;
        let disabled = self.disabled;
        let popup = popover(
            move |p| {
                p.trigger(
                    input_group::button(Icon::Calendar.boxed())
                        .square()
                        .disabled(disabled),
                )
            },
            move |_| {
                calendar()
                    .weekday_line_height(18.285715)
                    .selected(selected.map(Selection::single))
                    .caption_dropdown(caption)
                    .on_select(Event::Pick)
                    .into()
            },
        )
        .align(popover::Align::End)
        .align_offset(if caption { 0. } else { -8. })
        .side_offset(if caption { 8. } else { 10. })
        .padding(0)
        .focus_content(state.keyboard)
        .width(212)
        .open(state.open && !disabled)
        .on_open_change(Event::Open);
        let entry = input_group(
            crate::input(self.placeholder.clone())
                .value(state.text.clone())
                .text_size(self.text_size)
                .on_input(Event::Edit),
        )
        .addon(input_group::addon(popup).align(input_group::Align::InlineEnd))
        .disabled(disabled);
        let open = state.open;
        Keys {
            content: Element::from(entry),
            on_key: Box::new(move |key| {
                (!disabled && !open && key == Key::Named(Named::ArrowDown))
                    .then_some(Event::OpenFromInput)
            }),
        }
    }
}
impl<'a, M: 'static> Compose<M> for Input<'a, M> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, M, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}
impl<'a, M: 'static> From<Input<'a, M>> for View<'a, M> {
    fn from(input: Input<'a, M>) -> Self {
        View::new(input)
    }
}
impl<'a, M: 'static> From<Input<'a, M>> for Element<'a, M, Theme, Renderer> {
    fn from(input: Input<'a, M>) -> Self {
        component::component(input).boxed()
    }
}

/// Parse common relative phrases, weekdays and formatted dates against the supplied day.
pub fn relative(value: &str, today: Date) -> Option<Date> {
    let phrase = value.trim().to_lowercase();
    let offset = match phrase.as_str() {
        "today" => Some(0),
        "tomorrow" => Some(1),
        "yesterday" => Some(-1),
        "next week" => Some(7),
        _ => None,
    };
    if let Some(days) = offset {
        return today.checked_add_signed(Duration::days(days));
    }
    if let Some(next) = phrase.strip_prefix("next ") {
        let weekday = match next {
            "monday" => Some(Weekday::Mon),
            "tuesday" => Some(Weekday::Tue),
            "wednesday" => Some(Weekday::Wed),
            "thursday" => Some(Weekday::Thu),
            "friday" => Some(Weekday::Fri),
            "saturday" => Some(Weekday::Sat),
            "sunday" => Some(Weekday::Sun),
            _ => None,
        };
        if let Some(weekday) = weekday {
            let delta = (i64::from(weekday.num_days_from_monday())
                - i64::from(today.weekday().num_days_from_monday())
                - 1)
            .rem_euclid(7)
                + 1;
            return today.checked_add_signed(Duration::days(delta));
        }
    }
    let mut words = phrase.split_whitespace();
    if words.next() == Some("in")
        && let (Some(n), Some(unit), None) = (words.next(), words.next(), words.next())
    {
        let n: i64 = n.parse().ok()?;
        let multiplier = match unit {
            "day" | "days" => 1,
            "week" | "weeks" => 7,
            _ => return None,
        };
        return today.checked_add_signed(Duration::try_days(n.checked_mul(multiplier)?)?);
    }
    absolute(value, today)
}

fn absolute(value: &str, _: Date) -> Option<Date> {
    ["%Y-%m-%d", "%B %d, %Y", "%b %d, %Y", "%m/%d/%Y"]
        .iter()
        .find_map(|format| Date::parse_from_str(value.trim(), format).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interprets_relative_dates_across_month_and_year_boundaries() {
        let base = Date::from_ymd_opt(2025, 12, 31).unwrap();
        assert_eq!(relative("tomorrow", base), Date::from_ymd_opt(2026, 1, 1));
        assert_eq!(
            relative("next Wednesday", base),
            Date::from_ymd_opt(2026, 1, 7)
        );
        assert_eq!(
            relative("in 2 weeks", base),
            Date::from_ymd_opt(2026, 1, 14)
        );
        assert_eq!(absolute("02/29/2026", base), None);
    }
}
