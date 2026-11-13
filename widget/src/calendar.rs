//! A Nova calendar with selectable dates, ranges and controllable visible months.
mod date;

use crate::column::Column;
use crate::compose::{Compose, Inherited, Slot};
use crate::row::Row;
use crate::{button, column, container, row, space, text};
pub use chrono::{Datelike, Duration};
pub use date::{Date, System, Weekday, today};
use iced_core::alignment::Horizontal;
use iced_core::{Alignment, Element};
use iced_widget::component::{self, Component};
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Single,
    Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub start: Date,
    pub end: Option<Date>,
}

impl Selection {
    pub fn single(date: Date) -> Self {
        Self {
            start: date,
            end: None,
        }
    }
    pub fn range(a: Date, b: Date) -> Self {
        Self {
            start: a.min(b),
            end: Some(a.max(b)),
        }
    }
    fn contains(self, date: Date) -> bool {
        (self.start..=self.end.unwrap_or(self.start)).contains(&date)
    }
}

pub struct Calendar<'a, Message> {
    mode: Mode,
    system: System,
    selected: Option<Option<Selection>>,
    initial: Option<Selection>,
    month: Option<Date>,
    initial_month: Option<Date>,
    months: usize,
    caption: bool,
    week_numbers: bool,
    cell_size: f32,
    disabled: bool,
    unavailable: Option<Box<dyn Fn(Date) -> bool + 'a>>,
    annotation: Option<Box<dyn Fn(Date) -> String + 'a>>,
    on_change: Option<Box<dyn Fn(Option<Selection>) -> Message + 'a>>,
    on_month: Option<Box<dyn Fn(Date) -> Message + 'a>>,
}

pub fn calendar<'a, Message>() -> Calendar<'a, Message> {
    Calendar {
        mode: Mode::Single,
        system: System::Gregorian,
        selected: None,
        initial: None,
        month: None,
        initial_month: None,
        months: 1,
        caption: false,
        week_numbers: false,
        cell_size: 32.,
        disabled: false,
        unavailable: None,
        annotation: None,
        on_change: None,
        on_month: None,
    }
}

impl<'a, Message> Calendar<'a, Message> {
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
    pub fn system(mut self, system: System) -> Self {
        self.system = system;
        self
    }
    pub fn selected(mut self, selected: Option<Selection>) -> Self {
        self.selected = Some(selected);
        self
    }
    pub fn default_selected(mut self, selected: Selection) -> Self {
        self.initial = Some(selected);
        self
    }
    pub fn month(mut self, month: Date) -> Self {
        self.month = Some(month);
        self
    }
    pub fn default_month(mut self, month: Date) -> Self {
        self.initial_month = Some(month);
        self
    }
    pub fn number_of_months(mut self, months: usize) -> Self {
        self.months = months.clamp(1, 12);
        self
    }
    pub fn caption_dropdown(mut self, enabled: bool) -> Self {
        self.caption = enabled;
        self
    }
    pub fn week_numbers(mut self, enabled: bool) -> Self {
        self.week_numbers = enabled;
        self
    }
    pub fn cell_size(mut self, size: f32) -> Self {
        self.cell_size = size.max(16.);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn unavailable(mut self, f: impl Fn(Date) -> bool + 'a) -> Self {
        self.unavailable = Some(Box::new(f));
        self
    }
    pub fn annotate(mut self, f: impl Fn(Date) -> String + 'a) -> Self {
        self.annotation = Some(Box::new(f));
        self
    }
    pub fn on_select(mut self, f: impl Fn(Option<Selection>) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }
    pub fn on_month_change(mut self, f: impl Fn(Date) -> Message + 'a) -> Self {
        self.on_month = Some(Box::new(f));
        self
    }
}

#[derive(Default)]
pub struct State {
    selected: Option<Option<Selection>>,
    month: Option<Date>,
    caption: Option<Caption>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caption {
    Month(Date),
    Year(Date, i32),
}

#[derive(Clone, Debug)]
pub enum Event {
    Pick(Date),
    Month(Date),
    Caption(Option<Caption>),
}

impl<Message> Calendar<'_, Message> {
    fn selected_value(&self, state: &State) -> Option<Selection> {
        self.selected.or(state.selected).unwrap_or(self.initial)
    }
    fn visible_month(&self, state: &State) -> Date {
        self.system.first(
            self.month
                .or(state.month)
                .or(self.initial_month)
                .or_else(|| self.selected_value(state).map(|s| s.start))
                .unwrap_or_else(today),
        )
    }
    fn selectable(&self, day: Date) -> bool {
        !self.disabled && !self.unavailable.as_ref().is_some_and(|f| f(day))
    }
    fn choose(&self, old: Option<Selection>, day: Date) -> Option<Selection> {
        match (self.mode, old) {
            (Mode::Single, Some(s)) if s.start == day => None,
            (Mode::Range, Some(s)) if s.end.is_none() => Some(Selection::range(s.start, day)),
            _ => Some(Selection::single(day)),
        }
    }

    fn header(&self, month: Date, offset: usize) -> Slot<'static, Event> {
        let (year, number, _) = self.system.parts(month);
        let title: Slot<'static, Event> = if self.caption {
            let toggle = |label: String, width, caption| {
                button(
                    row![
                        text(label).size(13),
                        space::horizontal(),
                        text::muted("⌄").size(12)
                    ]
                    .spacing(2),
                )
                .style(button::Outline)
                .width(width)
                .height(30)
                .disabled(self.disabled)
                .on_press(Event::Caption(Some(caption)))
            };
            row![
                toggle(
                    self.system.month_name(number).to_owned(),
                    92,
                    Caption::Month(month)
                ),
                toggle(self.system.number(year), 64, Caption::Year(month, year - 6))
            ]
            .spacing(4)
            .into()
        } else {
            text::label(format!(
                "{} {}",
                self.system.month_name(number),
                self.system.number(year)
            ))
            .into()
        };
        let nav = |label, step, show| {
            if show && let Some(next) = self.system.shift(month, step - offset as i32) {
                Slot::from(
                    button(label)
                        .square()
                        .style(button::Ghost)
                        .width(28)
                        .height(28)
                        .disabled(self.disabled)
                        .on_press(Event::Month(next)),
                )
            } else {
                Slot::from(container("").width(28))
            }
        };
        row![
            nav("‹", -1, offset == 0),
            space::horizontal(),
            title,
            space::horizontal(),
            nav("›", 1, offset + 1 == self.months)
        ]
        .width(self.cell_size * (7 + usize::from(self.week_numbers)) as f32)
        .align_y(Alignment::Center)
        .into()
    }

    // Keep the chooser in the calendar's surface: the calendar may itself live in
    // a Date Picker popover, whose outside-click handler owns the overlay.
    fn caption_panel(&self, caption: Caption) -> Slot<'static, Event> {
        let mut rows = Vec::new();
        let mut choices = Vec::new();
        let (base, first, years) = match caption {
            Caption::Month(base) => (base, 1, false),
            Caption::Year(base, first) => (base, first, true),
        };
        let (year, month, _) = self.system.parts(base);
        if years {
            rows.push(
                row![
                    button("‹")
                        .style(button::Ghost)
                        .on_press(Event::Caption(Some(Caption::Year(base, first - 12)))),
                    space::horizontal(),
                    text::label(format!(
                        "{} – {}",
                        self.system.number(first),
                        self.system.number(first + 11)
                    )),
                    space::horizontal(),
                    button("›")
                        .style(button::Ghost)
                        .on_press(Event::Caption(Some(Caption::Year(base, first + 12))))
                ]
                .width(self.cell_size * 7.)
                .align_y(Alignment::Center)
                .into(),
            );
        }
        for n in first..first + 12 {
            let (y, m, label, active) = if years {
                (n, month, self.system.number(n), n == year)
            } else {
                {
                    let m = n as u32;
                    (year, m, self.system.month_name(m).to_owned(), m == month)
                }
            };
            let mut option = button(text(label).size(12))
                .width(self.cell_size * 7. / 3.)
                .height(34);
            if !active {
                option = option.style(button::Ghost);
            }
            if let Some(date) = self.system.date(y, m, 1) {
                option = option.on_press(Event::Month(date));
            } else {
                option = option.disabled(true);
            }
            choices.push(option.into());
            if choices.len() == 3 {
                rows.push(Row::with_children(std::mem::take(&mut choices)).into());
            }
        }
        rows.push(
            button("Back")
                .style(button::Ghost)
                .on_press(Event::Caption(None))
                .into(),
        );
        Column::with_children(rows).spacing(6).into()
    }

    fn grid(
        &self,
        month: Date,
        offset: usize,
        selected: Option<Selection>,
        today: Date,
    ) -> Slot<'static, Event> {
        let labels = match self.system {
            System::Gregorian => ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"],
            System::Persian => ["ش", "ی", "د", "س", "چ", "پ", "ج"],
        };
        let mut weekdays = Vec::new();
        if self.week_numbers {
            weekdays.push(
                container(text::muted("#"))
                    .width(self.cell_size)
                    .align_x(Horizontal::Center)
                    .into(),
            );
        }
        weekdays.extend(labels.map(|label| {
            container(text::muted(label).size(12))
                .width(self.cell_size)
                .align_x(Horizontal::Center)
                .into()
        }));
        let mut rows = vec![
            self.header(month, offset),
            Row::with_children(weekdays).into(),
        ];
        for week in self.system.weeks(month) {
            let mut cells = Vec::new();
            if self.week_numbers {
                cells.push(
                    container(text::muted(week[3].iso_week().week().to_string()).size(12))
                        .width(self.cell_size)
                        .align_x(Horizontal::Center)
                        .into(),
                );
            }
            for day in week {
                let (_, m, d) = self.system.parts(day);
                let outside = m != self.system.parts(month).1;
                let edge = selected.is_some_and(|s| s.start == day || s.end == Some(day));
                let unavailable = !self.selectable(day);
                let caption = self.system.number(d);
                let label = if outside || unavailable {
                    text::muted(caption)
                } else {
                    text(caption).size(14)
                };
                let label: Slot<'static, Event> =
                    if !outside && let Some(annotate) = &self.annotation {
                        column![label, text(annotate(day)).size(10)]
                            .spacing(1)
                            .align_x(Horizontal::Center)
                            .into()
                    } else {
                        label.into()
                    };
                let mut cell = button(label)
                    .square()
                    .width(self.cell_size)
                    .height(self.cell_size)
                    .disabled(unavailable)
                    .on_press(Event::Pick(day));
                if !edge {
                    cell = if selected.is_some_and(|s| s.contains(day)) || day == today {
                        cell.style(button::Secondary)
                    } else {
                        cell.style(button::Ghost)
                    };
                }
                cells.push(cell.into());
            }
            rows.push(Row::with_children(cells).align_y(Alignment::Center).into());
        }
        Column::with_children(rows).spacing(6).into()
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Calendar<'a, Message> {
    type State = State;
    type Event = Event;
    fn update(&self, state: &mut State, event: Event, _: &Renderer) -> Option<Message> {
        if self.disabled {
            return None;
        }
        let next_month = match event {
            Event::Pick(day) => {
                if !self.selectable(day) {
                    return None;
                }
                let selected = self.choose(self.selected_value(state), day);
                if self.month.is_none() {
                    state.month = Some(self.visible_month(state));
                }
                if self.selected.is_none() {
                    state.selected = Some(selected);
                }
                return self.on_change.as_ref().map(|change| change(selected));
            }
            Event::Month(date) => Some(date),
            Event::Caption(caption) => {
                state.caption = caption;
                return None;
            }
        }?;
        state.caption = None;
        if self.month.is_none() {
            state.month = Some(next_month);
        }
        self.on_month.as_ref().map(|f| f(next_month))
    }
    fn view(&self, state: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        if let Some(caption) = state.caption {
            return Element::from(container(self.caption_panel(caption)).padding(12));
        }
        let month = self.visible_month(state);
        let today = today();
        let grids = (0..self.months).filter_map(|i| {
            self.system
                .shift(month, i as i32)
                .map(|m| self.grid(m, i, self.selected_value(state), today))
        });
        Element::from(container(Row::with_children(grids).spacing(12).wrap()).padding(12))
    }
}

impl<'a, Message: 'static> Compose<Message> for Calendar<'a, Message> {
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
impl<'a, Message: 'static> From<Calendar<'a, Message>> for Slot<'a, Message> {
    fn from(calendar: Calendar<'a, Message>) -> Self {
        Self::new(calendar)
    }
}
impl<'a, Message: 'static> From<Calendar<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(calendar: Calendar<'a, Message>) -> Self {
        Box::new(calendar).compose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_reorders_reversed_endpoints_and_restarts_after_completion() {
        let start = Date::from_ymd_opt(2025, 12, 31).unwrap();
        let later = start + Duration::days(3);
        let range = calendar::<()>().mode(Mode::Range);
        let first = range.choose(None, later);
        let complete = range.choose(first, start);
        assert_eq!(complete, Some(Selection::range(start, later)));
        assert_eq!(
            range.choose(complete, start),
            Some(Selection::single(start))
        );
    }
}
