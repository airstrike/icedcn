```rust
use icedcn::card;
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Date, Datelike, Duration, Mode, Selection, today};

fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
}

pub fn range<'a, Message: 'static>(window_width: f32) -> card::Card<'a, Message> {
    let start = date(1, 12);
    let current = today();
    let picker = calendar()
        .mode(Mode::Range)
        .number_of_months(2)
        .stacked(window_width < 768.)
        .default_selected(Selection::range(start, start + Duration::days(30)))
        .unavailable(move |day| day > current || day.year() < 1900);
    card(picker).padding(0)
}
```
