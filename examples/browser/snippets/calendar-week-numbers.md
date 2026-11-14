```rust
use iced::Element;
use icedcn::{Theme, card};
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Date, Datelike, Selection, today};

fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
}

pub fn week_numbers<Message: 'static>() -> Element<'static, Message, Theme> {
    card(
        calendar()
            .default_selected(Selection::single(date(2, 3)))
            .week_numbers(true),
    )
    .padding(0)
    .into()
}
```
