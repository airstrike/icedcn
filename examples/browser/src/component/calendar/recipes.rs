use iced::widget::component::{Component, component};
use iced::{Element, Fill, Pixels, Widget};
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::calendar::{Date, Datelike, Duration, Mode, Selection, System, today};
use icedcn::widget::{
    Column, Row, button, calendar, column, container, field, input, input_group, rule, text,
};
use icedcn::{Size, Slot, Theme, card};

pub fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
}

fn bordered<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Element<'a, Message, Theme> {
    container(content)
        .padding(1)
        .style(|theme: &Theme| Style {
            background: Some(theme.neutral.base.into()),
            border: Border {
                width: 1.,
                radius: 10.0.into(),
                color: theme.neutral.border,
            },
            ..Style::default()
        })
        .into()
}

pub fn demo<Message: 'static>() -> Element<'static, Message, Theme> {
    bordered(
        calendar()
            .default_selected(Selection::single(today()))
            .caption_dropdown(true),
    )
}
pub fn hijri<Message: 'static>() -> Element<'static, Message, Theme> {
    bordered(
        calendar()
            .system(System::Persian)
            .cell_size(32.)
            .padding(12.)
            .default_selected(Selection::single(
                Date::from_ymd_opt(2025, 6, 12).unwrap_or_else(today),
            )),
    )
}
pub fn basic<Message: 'static>() -> Element<'static, Message, Theme> {
    bordered(calendar())
}
pub fn range<Message: 'static>(wide: Option<bool>) -> Element<'static, Message, Theme> {
    let mut picker = calendar()
        .weekday_line_height(18.285715)
        .mode(Mode::Range)
        .number_of_months(2)
        .default_selected(Selection::range(
            date(1, 12),
            date(1, 12) + Duration::days(30),
        ))
        .unavailable(|day| day > today() || day.year() < 1900);
    if let Some(wide) = wide {
        picker = picker.stacked(!wide);
    }
    card(picker).boxed()
}
pub fn caption<Message: 'static>() -> Element<'static, Message, Theme> {
    bordered(calendar().caption_dropdown(true))
}
pub fn booked_dates<Message: 'static>() -> Element<'static, Message, Theme> {
    card(
        calendar()
            .weekday_line_height(18.285715)
            .default_selected(Selection::single(date(2, 3)))
            .unavailable(|day| (date(2, 12)..=date(2, 26)).contains(&day))
            .strike_unavailable(true),
    )
    .boxed()
}
pub fn custom_days<Message: 'static>(wide: bool) -> Element<'static, Message, Theme> {
    card(
        calendar()
            .weekday_line_height(18.285715)
            .mode(Mode::Range)
            .caption_dropdown(true)
            .long_months(true)
            .cell_size(if wide { 48. } else { 40. })
            .default_selected(Selection::range(date(12, 8), date(12, 18)))
            .annotate(|day| {
                if day.weekday().number_from_monday() >= 6 {
                    "$120"
                } else {
                    "$100"
                }
                .to_owned()
            }),
    )
    .boxed()
}
pub fn week_numbers<Message: 'static>() -> Element<'static, Message, Theme> {
    card(
        calendar()
            .weekday_line_height(18.285715)
            .default_selected(Selection::single(date(2, 3)))
            .week_numbers(true),
    )
    .boxed()
}

pub fn presets<Message: 'static>() -> Element<'static, Message, Theme> {
    component(Presets).boxed()
}
struct Presets;
#[derive(Default)]
struct Dates {
    value: Option<Option<Selection>>,
    month: Option<Date>,
}
#[derive(Clone, Debug)]
enum Edit {
    Pick(Option<Selection>),
    Show(Date),
    Offset(i64),
}
impl<'a, Message: 'static> Component<'a, Message, Theme> for Presets {
    type State = Dates;
    type Event = Edit;
    fn update(&self, state: &mut Dates, edit: Edit, _: &iced::Renderer) -> Option<Message> {
        match edit {
            Edit::Pick(value) => state.value = Some(value),
            Edit::Show(month) => state.month = Some(month),
            Edit::Offset(days) => {
                let day = today() + Duration::days(days);
                state.value = Some(Some(Selection::single(day)));
                state.month = Some(day);
            }
        }
        None
    }
    fn view(&self, state: &Dates) -> impl Widget<Edit, Theme> + 'a {
        let picker = calendar()
            .weekday_line_height(18.285715)
            .padding(0.)
            .cell_size(38.)
            .fixed_weeks(true)
            .selected(state.value.unwrap_or(Some(Selection::single(date(2, 12)))))
            .month(state.month.unwrap_or_else(today))
            .on_select(Edit::Pick)
            .on_month_change(Edit::Show);
        let presets = [
            ("Today", 0),
            ("Tomorrow", 1),
            ("In 3 days", 3),
            ("In a week", 7),
            ("In 2 weeks", 14),
        ];
        let actions = presets.chunks(3).map(|chunk| {
            Row::with_children(chunk.iter().map(|&(label, days)| {
                button(label)
                    .style(button::Outline)
                    .size(Size::Sm)
                    .width(Fill)
                    .on_press(Edit::Offset(days))
                    .into()
            }))
            .spacing(8)
            .into()
        });
        card(
            column![
                container(picker).padding(12).center_x(Fill),
                card::footer_with_spacing(Column::with_children(actions).spacing(8), 12.),
            ]
            .width(300),
        )
        .boxed()
    }
}

pub fn time<Message: 'static>() -> Element<'static, Message, Theme> {
    component(Times).boxed()
}
struct Times;
struct Hours {
    start: String,
    end: String,
}
impl Default for Hours {
    fn default() -> Self {
        Self {
            start: "10:30:00".into(),
            end: "12:30:00".into(),
        }
    }
}
#[derive(Clone, Debug)]
enum Time {
    Start(String),
    End(String),
}
// Allow partial edits, while rejecting values outside a 24-hour clock.
fn valid_time(value: &str) -> bool {
    if value.len() > 8 || !value.bytes().all(|b| b.is_ascii_digit() || b == b':') {
        return false;
    }
    let parts: Vec<_> = value.split(':').collect();
    parts.len() <= 3
        && parts.iter().enumerate().all(|(i, p)| {
            p.len() <= 2
                && (p.is_empty()
                    || p.parse::<u32>()
                        .is_ok_and(|n| n < if i == 0 { 24 } else { 60 }))
        })
}
impl<'a, Message: 'static> Component<'a, Message, Theme> for Times {
    type State = Hours;
    type Event = Time;
    fn update(&self, state: &mut Hours, edit: Time, _: &iced::Renderer) -> Option<Message> {
        match edit {
            Time::Start(value) if valid_time(&value) => state.start = value,
            Time::End(value) if valid_time(&value) => state.end = value,
            _ => {}
        }
        None
    }
    fn view(&self, state: &Hours) -> impl Widget<Time, Theme> + 'a {
        let start = field(
            input_group(
                input("HH:mm:ss")
                    .value(state.start.clone())
                    .on_input(Time::Start),
            )
            .addon(input_group::addon(clock().boxed())),
        )
        .label(text::label("Start Time").line_height(Pixels(19.25)));
        let end = field(
            input_group(
                input("HH:mm:ss")
                    .value(state.end.clone())
                    .on_input(Time::End),
            )
            .addon(input_group::addon(clock().boxed())),
        )
        .label(text::label("End Time").line_height(Pixels(19.25)));
        card(
            column![
                container(
                    calendar()
                        .weekday_line_height(18.285715)
                        .padding(0.)
                        .default_selected(Selection::single(date(today().month(), 12)))
                )
                .padding(12),
                rule::horizontal(1).boxed(),
                container(field::group([start.into(), end.into()])).padding(12),
            ]
            .width(220),
        )
        .boxed()
    }
}

fn clock<'a>() -> iced::widget::Svg<'a, Theme> {
    use iced::widget::svg;
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4-2"/></svg>"#))
        .width(16).height(16).style(|theme: &Theme, _| svg::Style { color: Some(theme.muted.foreground.into()) })
}
