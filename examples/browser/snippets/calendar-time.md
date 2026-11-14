```rust
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
```
