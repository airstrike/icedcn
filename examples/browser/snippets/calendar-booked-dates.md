```rust
use icedcn::card;
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Date, Datelike, Selection, today};

fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
}

pub fn booked_dates<Message: 'static>() -> card::Card<'static, Message> {
    let booked = date(2, 12)..=date(2, 26);
    card(
        calendar()
            .default_selected(Selection::single(date(2, 3)))
            .unavailable(move |day| booked.contains(&day))
            .strike_unavailable(true),
    )
    .padding(0)
}
```
