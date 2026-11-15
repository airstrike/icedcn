```rust
use icedcn::card;
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Date, Datelike, Mode, Selection, today};

fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
}

pub fn custom_days<Message: 'static>(window_width: f32) -> card::Card<'static, Message> {
    card(
        calendar()
            .mode(Mode::Range)
            .caption_dropdown(true)
            .long_months(true)
            .cell_size(if window_width >= 768. { 48. } else { 40. })
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
    .padding(0)
}
```
