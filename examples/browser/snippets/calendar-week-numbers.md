```rust
use icedcn::card;
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Date, Datelike, Selection, today};

fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
}

pub fn week_numbers<'a, Message: 'static>() -> card::Card<'a, Message> {
    card(
        calendar()
            .default_selected(Selection::single(date(2, 3)))
            .week_numbers(true),
    )
    .padding(0)
}
```
