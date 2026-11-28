//! A Nova calendar with selectable dates, ranges and controllable visible months.
mod date;
mod grid;
#[cfg(feature = "svg")]
mod icon;
pub mod persian;
mod week;

use crate::Weight;
use crate::column::Column;
use crate::compose::{Compose, Inherited, View};
use crate::row::Row;
use crate::{button, column, container, dropdown_menu, menu, row, space, text};
pub use chrono::{Datelike, Duration};
pub use date::{Date, System, Weekday, today};
use iced_core::alignment::Horizontal;
use iced_core::font;
use iced_core::keyboard::key::Named;
use iced_core::text::LineHeight;
use iced_core::{Alignment, Element, Padding};
use iced_widget::component::{self, Component};
use iced_widget::text::StyleFn;
use iced_widget::{Renderer, Widget};
use iced_widget::{rich_text, span};
use icedcn_core::Theme;
use icedcn_core::theme;
use icedcn_core::theme::calendar::Day;
use icedcn_core::theme::container::PageBackground;
use icedcn_core::theme::tweak::{Token, border, radius};

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
    padding: f32,
    weekday_line_height: Option<f32>,
    bordered: bool,
    fixed_weeks: bool,
    stacked: Option<bool>,
    long_months: bool,
    strike_unavailable: bool,
    required: bool,
    min: u32,
    max: u32,
    exclude_unavailable: bool,
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
        cell_size: 28.,
        padding: 8.,
        weekday_line_height: None,
        bordered: false,
        fixed_weeks: false,
        stacked: None,
        long_months: false,
        strike_unavailable: false,
        required: false,
        min: 0,
        max: 0,
        exclude_unavailable: false,
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
    /// Space around the months. Nova uses 8 logical pixels; card content can use 0.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.);
        self
    }
    /// Give a standalone calendar a rounded border and an opaque background.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
    /// Override the weekday line box, normally inherited from the enclosing Card.
    pub fn weekday_line_height(mut self, height: f32) -> Self {
        self.weekday_line_height = Some(height.max(12.8));
        self
    }
    /// Reserve six week rows even in shorter months.
    pub fn fixed_weeks(mut self, fixed: bool) -> Self {
        self.fixed_weeks = fixed;
        self
    }
    /// Set the month direction from the application's window breakpoint.
    /// Without an override, months wrap when the available content width is small.
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = Some(stacked);
        self
    }
    /// Use full month names in the month dropdown instead of abbreviations.
    pub fn long_months(mut self, long: bool) -> Self {
        self.long_months = long;
        self
    }
    /// Cross out unavailable dates, as in a booking calendar.
    pub fn strike_unavailable(mut self, strike: bool) -> Self {
        self.strike_unavailable = strike;
        self
    }
    /// Keep at least one date selected: picking the selected date again
    /// leaves it selected instead of clearing it.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    /// The fewest nights a range spans. A shorter range restarts from the
    /// picked date.
    pub fn min(mut self, nights: u32) -> Self {
        self.min = nights;
        self
    }
    /// The most nights a range spans. A longer range restarts from the
    /// picked date.
    pub fn max(mut self, nights: u32) -> Self {
        self.max = nights;
        self
    }
    /// Restart a range from the picked date when it would span an
    /// unavailable date.
    pub fn exclude_unavailable(mut self, exclude: bool) -> Self {
        self.exclude_unavailable = exclude;
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
    focused: Option<Date>,
    focus_generation: u64,
}

#[derive(Clone, Debug)]
pub enum Event {
    Pick(Date),
    Month(Date),
    Navigate(Date, Named, bool),
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
    fn navigable_month(&self, day: Date) -> bool {
        !self.caption || {
            let year = self.system.parts(day).0;
            let current = self.system.parts(today()).0;
            (current - 100..=current).contains(&year)
        }
    }
    fn selectable(&self, day: Date) -> bool {
        !self.disabled
            && self.navigable_month(day)
            && !self.unavailable.as_ref().is_some_and(|f| f(day))
    }
    /// DayPicker's `useSingle` and `useRange` with `addToRange`.
    fn choose(&self, old: Option<Selection>, day: Date) -> Option<Selection> {
        let open = |start| Some(Selection { start, end: None });
        if self.mode == Mode::Single {
            return match old {
                Some(s) if s.start == day && !self.required => None,
                _ => Some(Selection::single(day)),
            };
        }
        let range = match old {
            None if self.min > 0 => open(day),
            None => Some(Selection::range(day, day)),
            Some(Selection { start, end: None }) if start == day => {
                if self.required {
                    open(start)
                } else {
                    None
                }
            }
            Some(Selection { start, end: None }) => Some(Selection::range(start, day)),
            Some(Selection {
                start,
                end: Some(end),
            }) => {
                if start == day && end == day {
                    if self.required { old } else { None }
                } else if start == day || end == day {
                    if self.min > 0 {
                        open(day)
                    } else {
                        Some(Selection::range(day, day))
                    }
                } else if day < start {
                    Some(Selection::range(day, end))
                } else {
                    Some(Selection::range(start, day))
                }
            }
        };
        let Some(Selection {
            start,
            end: Some(end),
        }) = range
        else {
            return range;
        };
        let nights = (end - start).num_days();
        if self.max > 0 && nights > i64::from(self.max)
            || self.min > 1 && nights < i64::from(self.min)
        {
            return open(day);
        }
        if self.exclude_unavailable
            && let Some(unavailable) = &self.unavailable
            && start.iter_days().take_while(|d| *d <= end).any(unavailable)
        {
            return open(day);
        }
        range
    }

    fn header(
        &self,
        month: Date,
        offset: usize,
        targets: &mut Vec<Option<Date>>,
    ) -> View<'static, Event> {
        let (year, number, _) = self.system.parts(month);
        let system = self.system;
        let disabled = self.disabled;
        let previous = offset == 0;
        let next = if self.stacked == Some(true) {
            offset == 0
        } else {
            offset + 1 == self.months
        };
        let rtl = system == System::Persian;
        let left = if rtl { next } else { previous };
        let right = if rtl { previous } else { next };
        let nav = |label, step, show| {
            if show && let Some(next) = system.shift(month, step - offset as i32) {
                View::from(
                    button(chevron(label))
                        .square()
                        .style(button::Ghost)
                        .width(self.cell_size)
                        .height(self.cell_size)
                        .disabled(disabled || !self.navigable_month(next))
                        .on_press(Event::Month(next)),
                )
            } else {
                View::from(container("").width(self.cell_size).height(self.cell_size))
            }
        };
        if left
            && !disabled
            && system
                .shift(month, (if rtl { 1 } else { -1 }) - offset as i32)
                .is_some_and(|d| self.navigable_month(d))
        {
            targets.push(None);
        }
        let title: View<'static, Event> = if self.caption {
            let dropdown = |label: String, choices: Vec<(String, Date)>| {
                let selected = choices
                    .iter()
                    .position(|(_, date)| *date == month)
                    .unwrap_or(0);
                dropdown_menu(
                    move |handle| {
                        handle.trigger(
                            button(
                                row![
                                    text::Text::new(label.clone()).size(14),
                                    container(chevron("⌄")).style(|t: &Theme| {
                                        theme::container::Style {
                                            text_color: Some(t.muted.foreground),
                                            ..Default::default()
                                        }
                                    }),
                                ]
                                .spacing(4),
                            )
                            .style(button::Outline)
                            .map_style(|mut style| {
                                style.background = None;
                                style.border.radius = 8.0.into();
                                style
                            })
                            .height(26)
                            .padding(Padding {
                                top: 1.,
                                right: 4.,
                                bottom: 1.,
                                left: 6.,
                            })
                            .translate(false)
                            .disabled(disabled),
                        )
                    },
                    move |_| {
                        Column::with_children(choices.iter().map(|(label, date)| {
                            menu::item(label.clone())
                                .on_select(Event::Month(*date))
                                .into()
                        }))
                        .into()
                    },
                )
                .initial_choice(selected, 240.)
                .align(dropdown_menu::Align::Start)
                .width(144)
            };
            let month_label = |number| {
                let label = system.month_name(number);
                if self.long_months || system == System::Persian {
                    label.to_owned()
                } else {
                    label.chars().take(3).collect()
                }
            };
            let label = month_label(number);
            let months = (1..=12)
                .filter_map(|m| system.date(year, m, 1).map(|d| (month_label(m), d)))
                .collect();
            let current_year = system.parts(today()).0;
            let years = (current_year - 100..=current_year)
                .filter_map(|y| system.date(y, number, 1).map(|d| (system.number(y), d)))
                .collect();
            if !disabled {
                targets.extend([None, None]);
            }
            row![
                dropdown(label, months),
                dropdown(system.number(year), years)
            ]
            .spacing(6)
            .into()
        } else {
            let mut label = iced_widget::text(format!(
                "{} {}",
                system.month_name(number),
                system.number(year)
            ))
            .size(14)
            .weight(font::Weight::Medium)
            .line_height(LineHeight::Relative(1.4285714));
            if system == System::Persian {
                label = label.font(persian::FAMILY);
            }
            label.into()
        };
        if right
            && !disabled
            && system
                .shift(month, (if rtl { -1 } else { 1 }) - offset as i32)
                .is_some_and(|d| self.navigable_month(d))
        {
            targets.push(None);
        }
        row![
            nav("‹", if rtl { 1 } else { -1 }, left),
            space::horizontal(),
            title,
            space::horizontal(),
            nav("›", if rtl { -1 } else { 1 }, right)
        ]
        .width(self.cell_size * (7 + usize::from(self.week_numbers)) as f32)
        .height(self.cell_size)
        .align_y(Alignment::Center)
        .into()
    }

    fn grid(
        &self,
        month: Date,
        offset: usize,
        selected: Option<Selection>,
        today: Date,
        targets: &mut Vec<Option<Date>>,
        in_month: &mut Vec<usize>,
    ) -> View<'static, Event> {
        let weekday_line_height = self.weekday_line_height.unwrap_or(22.4);
        let mut labels = match self.system {
            System::Gregorian => ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"],
            System::Persian => ["ش", "۱ش", "۲ش", "۳ش", "۴ش", "۵ش", "ج"],
        };
        if self.system == System::Persian {
            labels.reverse();
        }
        let header = self.header(month, offset, targets);
        let mut weekdays = Vec::new();
        if self.week_numbers {
            weekdays.push(container("").width(self.cell_size).into());
        }
        weekdays.extend(labels.map(|label| {
            let mut label = iced_widget::text(label)
                .size(12.8)
                .line_height(LineHeight::Absolute(weekday_line_height.into()))
                .style(|t: &Theme| {
                    theme::text::Style {
                        color: Some(t.muted.foreground),
                    }
                    .into()
                });
            if self.system == System::Persian {
                label = label.font(persian::FAMILY);
            }
            container(label)
                .width(self.cell_size)
                .align_x(Horizontal::Center)
                .into()
        }));
        let mut rows = vec![Row::with_children(weekdays).into()];
        let mut weeks = self.system.weeks(month);
        while self.fixed_weeks && weeks.len() < 6 {
            let Some(last) = weeks.last() else { break };
            let Some(start) = last[0].checked_add_signed(Duration::weeks(1)) else {
                break;
            };
            if start.checked_add_signed(Duration::days(6)).is_none() {
                break;
            }
            weeks.push(std::array::from_fn(|i| start + Duration::days(i as i64)));
        }
        let rtl = self.system == System::Persian;
        for week in weeks {
            let mut days = Vec::new();
            let mut cells: Vec<Element<'static, Event, Theme, Renderer>> = Vec::new();
            if self.week_numbers {
                days.push(None);
                cells.push(
                    View::from(
                        container(
                            text::muted(format!("{:02}", date::week_number(week[0])))
                                .size(12.8)
                                .line_height(LineHeight::Absolute(weekday_line_height.into())),
                        )
                        .width(self.cell_size)
                        .height(self.cell_size)
                        .center_y(self.cell_size)
                        .align_x(Horizontal::Center),
                    )
                    .compose(),
                );
            }
            for weekday in 0..7 {
                let index = if rtl { 6 - weekday } else { weekday };
                let day = week[index];
                if !self.navigable_month(day) {
                    days.push(None);
                    cells.push(
                        View::from(container("").width(self.cell_size).height(self.cell_size))
                            .compose(),
                    );
                    continue;
                }
                let (_, m, d) = self.system.parts(day);
                let unavailable = !self.selectable(day);
                let range = self.mode == Mode::Range;
                let complete = selected.and_then(|s| s.end.map(|end| (s.start, end)));
                let paint = Day {
                    selected: selected.is_some_and(|s| s.contains(day)),
                    range_start: range && complete.is_some_and(|(start, _)| start == day),
                    range_end: range && complete.is_some_and(|(_, end)| end == day),
                    range_middle: range
                        && complete.is_some_and(|(start, end)| start < day && day < end),
                    today: day == today,
                    outside: m != self.system.parts(month).1,
                    disabled: unavailable,
                    struck: unavailable && self.strike_unavailable,
                    first: index == 0,
                    last: index == 6,
                    rtl,
                };
                let mut label = rich_text([span(self.system.number(d))
                    .strikethrough(unavailable && self.strike_unavailable)])
                .size(14)
                .line_height(LineHeight::Relative(1.));
                if self.system == System::Persian {
                    label = label.font(persian::FAMILY);
                }
                let label: View<'static, Event> = label.into();
                let label: View<'static, Event> = if !paint.outside
                    && let Some(annotate) = &self.annotation
                {
                    column![
                        label,
                        text::Text::new(annotate(day))
                            .style(Box::new(move |t: &Theme| theme::text::Style {
                                color: Some(paint.foreground(t).scale_alpha(0.7))
                            }
                            .into())
                                as StyleFn<'static, Theme>)
                            .size(12)
                            .line_height(LineHeight::Absolute(16.0.into()))
                    ]
                    .spacing(4)
                    .align_x(Horizontal::Center)
                    .into()
                } else {
                    label
                };
                if !unavailable {
                    if !paint.outside {
                        in_month.push(targets.len());
                    }
                    targets.push(Some(day));
                }
                days.push(Some(paint));
                cells.push(
                    View::from(
                        button(label)
                            .square()
                            .weight(Weight::Normal)
                            .width(self.cell_size)
                            .height(self.cell_size)
                            .style(paint)
                            .disabled(unavailable)
                            .on_press(Event::Pick(day)),
                    )
                    .compose(),
                );
            }
            rows.push(
                week::Week {
                    days,
                    children: cells,
                    size: self.cell_size,
                }
                .boxed()
                .into(),
            );
        }
        column![header, Column::with_children(rows).spacing(8)]
            .spacing(16)
            .into()
    }

    fn navigate(&self, mut day: Date, key: Named, shift: bool) -> Option<Date> {
        // DayPicker retries the same unit (a disabled week keeps its weekday),
        // with at most 366 attempts. Home/End keep their week-boundary target.
        for _ in 0..=365 {
            let weekday = match self.system {
                System::Gregorian => day.weekday().num_days_from_sunday(),
                System::Persian => (day.weekday().num_days_from_sunday() + 1) % 7,
            };
            let direction = match key {
                Named::ArrowLeft => {
                    if self.system == System::Persian {
                        1
                    } else {
                        -1
                    }
                }
                Named::ArrowRight => {
                    if self.system == System::Persian {
                        -1
                    } else {
                        1
                    }
                }
                Named::ArrowUp | Named::PageUp | Named::Home => -1,
                Named::ArrowDown | Named::PageDown | Named::End => 1,
                _ => return None,
            };
            let mut target = match key {
                Named::ArrowLeft | Named::ArrowRight if shift => {
                    self.system.shift_day(day, direction)
                }
                Named::ArrowUp | Named::ArrowDown if shift => {
                    self.system.shift_day(day, direction * 12)
                }
                Named::ArrowLeft | Named::ArrowRight => {
                    day.checked_add_signed(Duration::days(direction as i64))
                }
                Named::ArrowUp | Named::ArrowDown => {
                    day.checked_add_signed(Duration::weeks(direction as i64))
                }
                Named::Home => day.checked_sub_signed(Duration::days(i64::from(weekday))),
                Named::End => day.checked_add_signed(Duration::days(6 - i64::from(weekday))),
                Named::PageUp | Named::PageDown => self
                    .system
                    .shift_day(day, direction * if shift { 12 } else { 1 }),
                _ => None,
            }?;
            if self.caption {
                let year = self.system.parts(today()).0;
                let start = self.system.date(year - 100, 1, 1)?;
                let end = self.system.date(year + 1, 1, 1)?.pred_opt()?;
                target = target.clamp(start, end);
            }
            if self.selectable(target) {
                return Some(target);
            }
            if target == day {
                return None;
            }
            day = target;
        }
        None
    }
}

impl<'a, Message: 'static> Component<'a, Message, Theme, Renderer> for Calendar<'a, Message> {
    type State = State;
    type Event = Event;
    fn diff(&mut self, state: &mut State) {
        if state.selected.is_none() {
            state.selected = Some(self.initial);
        }
        if state.month.is_none() {
            state.month = Some(self.visible_month(state));
        }
    }
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
            Event::Month(date) => self.navigable_month(date).then_some(date),
            Event::Navigate(day, key, shift) => {
                let target = self.navigate(day, key, shift)?;
                state.focused = Some(target);
                state.focus_generation = state.focus_generation.wrapping_add(1);
                let first = self.visible_month(state);
                let last = self
                    .system
                    .shift(first, self.months as i32)
                    .unwrap_or(Date::MAX);
                if target < first {
                    Some(self.system.first(target))
                } else if target >= last {
                    self.system.shift(target, 1 - self.months as i32)
                } else {
                    return None;
                }
            }
        }?;
        if self.month.is_none() {
            state.month = Some(next_month);
        }
        self.on_month.as_ref().map(|f| f(next_month))
    }
    fn view(&self, state: &State) -> impl Widget<Event, Theme, Renderer> + 'a {
        let month = self.visible_month(state);
        let today = today();
        let selected = self.selected_value(state);
        let mut targets = Vec::new();
        let mut in_month = Vec::new();
        let grids: Vec<_> = (0..self.months)
            .filter_map(|i| {
                self.system
                    .shift(month, i as i32)
                    .map(|m| self.grid(m, i, selected, today, &mut targets, &mut in_month))
            })
            .collect();
        let content: View<'static, Event> = match self.stacked {
            Some(true) => Column::with_children(grids).spacing(16).into(),
            Some(false) => Row::with_children(grids)
                .spacing(16)
                .align_y(Alignment::Start)
                .into(),
            None => Row::with_children(grids)
                .spacing(16)
                .align_y(Alignment::Start)
                .wrap()
                .into(),
        };
        let preferred = in_month
            .iter()
            .copied()
            .filter(|&i| targets[i].is_some_and(|d| selected.is_some_and(|s| s.contains(d))))
            .min_by_key(|&i| targets[i])
            .or_else(|| {
                in_month
                    .iter()
                    .copied()
                    .find(|&i| targets[i] == Some(today))
            })
            .or_else(|| in_month.iter().copied().min_by_key(|&i| targets[i]));
        let mut surface =
            container(content).padding(self.padding + if self.bordered { 1. } else { 0. });
        if self.bordered {
            surface = surface.style(
                PageBackground + border::Color(Token::Border) + border::Width(1.) + radius(10.),
            );
        }
        grid::Grid {
            content: surface.into(),
            targets,
            preferred,
            in_month,
            focused: state.focused,
            generation: state.focus_generation,
        }
    }
}

impl<'a, Message: 'static> Compose<Message> for Calendar<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.disabled |= from.is_disabled();
        if from.card_spacing().is_some() && !self.bordered {
            self.weekday_line_height.get_or_insert(12.8 * 20. / 14.);
        }
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        component::component(*self).boxed()
    }
}
impl<'a, Message: 'static> From<Calendar<'a, Message>> for View<'a, Message> {
    fn from(calendar: Calendar<'a, Message>) -> Self {
        Self::new(calendar)
    }
}
impl<'a, Message: 'static> From<Calendar<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(calendar: Calendar<'a, Message>) -> Self {
        Box::new(calendar).compose()
    }
}

fn chevron(direction: &str) -> View<'static, Event> {
    #[cfg(feature = "svg")]
    {
        icon::Chevron(direction.chars().next().unwrap_or('⌄'))
            .boxed()
            .into()
    }
    #[cfg(not(feature = "svg"))]
    {
        text::Text::new(direction.to_owned()).size(16).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_allows_one_day_and_edits_existing_endpoints() {
        let day = |month, day| Date::from_ymd_opt(2026, month, day).unwrap();
        let start = day(1, 12);
        let mid = day(1, 20);
        let end = day(2, 11);
        let old = Some(Selection::range(start, end));
        let picker = calendar::<()>().mode(Mode::Range);
        for (previous, clicked, expected) in [
            (None, mid, Some(Selection::range(mid, mid))),
            (Some(Selection::range(mid, mid)), mid, None),
            (
                Some(Selection::single(mid)),
                start,
                Some(Selection::range(start, mid)),
            ),
            (Some(Selection::single(mid)), mid, None),
            (
                Some(Selection::single(mid)),
                end,
                Some(Selection::range(mid, end)),
            ),
            (old, mid, Some(Selection::range(start, mid))),
            (old, start, Some(Selection::range(start, start))),
            (old, end, Some(Selection::range(end, end))),
            (old, day(1, 1), Some(Selection::range(day(1, 1), end))),
            (old, day(2, 20), Some(Selection::range(start, day(2, 20)))),
        ] {
            assert_eq!(picker.choose(previous, clicked), expected);
        }
    }
    #[test]
    fn range_constraints_follow_add_to_range() {
        let day = |d| Date::from_ymd_opt(2026, 1, d).unwrap();
        let open = |d| {
            Some(Selection {
                start: day(d),
                end: None,
            })
        };
        let range = |a, b| Some(Selection::range(day(a), day(b)));
        let picker = || calendar::<()>().mode(Mode::Range);
        let required = picker().required(true);
        for (previous, clicked, expected) in [
            (range(5, 5), 5, range(5, 5)),
            (open(5), 5, open(5)),
            (open(5), 3, range(3, 5)),
        ] {
            assert_eq!(required.choose(previous, day(clicked)), expected);
        }
        assert_eq!(picker().choose(open(5), day(5)), None);
        // A range shorter than `min` nights stays open at the picked date.
        let min = picker().min(3);
        for (previous, clicked, expected) in [
            (None, 5, open(5)),
            (open(5), 7, open(7)),
            (open(5), 8, range(5, 8)),
            (range(5, 8), 5, open(5)),
            (range(5, 8), 8, open(8)),
            (range(5, 8), 9, range(5, 9)),
        ] {
            assert_eq!(min.choose(previous, day(clicked)), expected);
        }
        let max = picker().max(4);
        assert_eq!(max.choose(range(5, 6), day(9)), range(5, 9));
        assert_eq!(max.choose(range(5, 6), day(10)), open(10));
        let blocked = picker()
            .unavailable(|d| d == Date::from_ymd_opt(2026, 1, 7).unwrap())
            .exclude_unavailable(true);
        assert_eq!(blocked.choose(range(5, 5), day(9)), open(9));
        assert_eq!(blocked.choose(range(8, 8), day(9)), range(8, 9));
        let single = calendar::<()>();
        assert_eq!(single.choose(Some(Selection::single(day(5))), day(5)), None);
        assert_eq!(
            single
                .required(true)
                .choose(Some(Selection::single(day(5))), day(5)),
            Some(Selection::single(day(5)))
        );
    }
    #[test]
    fn keyboard_months_clamp_and_disabled_weeks_keep_the_weekday() {
        let day = Date::from_ymd_opt(2024, 2, 29).unwrap();
        let picker = calendar::<()>();
        assert_eq!(
            picker.navigate(day, Named::PageDown, true),
            Date::from_ymd_opt(2025, 2, 28)
        );
        assert_eq!(
            picker.navigate(day, Named::ArrowRight, true),
            Date::from_ymd_opt(2024, 3, 29)
        );
        let blocked = day + Duration::weeks(1);
        let picker = picker.unavailable(move |d| d == blocked);
        assert_eq!(
            picker.navigate(day, Named::ArrowDown, false),
            Some(day + Duration::weeks(2))
        );
        assert_eq!(
            calendar::<()>()
                .unavailable(|_| true)
                .navigate(day, Named::ArrowRight, false),
            None
        );
    }
}
