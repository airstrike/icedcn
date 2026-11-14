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
pub fn example<Message: 'static>() -> Element<'static, Message, Theme> {
    card(
        calendar()
            .weekday_line_height(18.285715)
            .default_selected(Selection::single(date(2, 3)))
            .unavailable(|day| (date(2, 12)..=date(2, 26)).contains(&day))
            .strike_unavailable(true),
    )
    .boxed()
}
```
