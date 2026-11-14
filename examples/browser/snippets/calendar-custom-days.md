```rust
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Pixels, Widget};
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::calendar::{Date, Datelike, Duration, Mode, Selection, System, today};
use icedxdy::widget::{
    Column, Row, button, calendar, column, container, field, input, input_group, rule, text,
};
use icedxdy::{Size, Slot, Theme, card};

pub fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
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

struct Example { window_width: f32 }
#[derive(Clone, Debug)]
enum Message { Width(f32) }
impl Example {
    fn new() -> (Self, iced::Task<Message>) {
        use iced::window;
        (Self { window_width: 1024. }, window::oldest().and_then(window::size)
            .map(|size| Message::Width(size.width)))
    }
    fn update(&mut self, Message::Width(width): Message) { self.window_width = width; }
    fn subscription(&self) -> iced::Subscription<Message> {
        use iced::window;
        window::resize_events().map(|(_, size)| Message::Width(size.width))
    }
    fn view(&self) -> Element<'static, Message, Theme> {
        custom_days(self.window_width >= 768.)
    }
}
fn main() -> iced::Result {
    iced::application(Example::new, Example::update, Example::view)
        .subscription(Example::subscription)
        .theme(|_: &Example| Theme::light()).run()
}
```
