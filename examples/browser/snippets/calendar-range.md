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
        range(Some(self.window_width >= 768.))
    }
}
fn main() -> iced::Result {
    iced::application(Example::new, Example::update, Example::view)
        .subscription(Example::subscription)
        .theme(|_: &Example| Theme::light()).run()
}
```
