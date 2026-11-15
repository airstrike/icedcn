```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::calendar::{Date, Datelike, Duration, Mode, Selection, today};
use icedcn::widget::{date_picker, field, text};

pub fn view<'a, M: 'static>(window_width: f32) -> Slot<'a, M>
where
    M: Clone,
{
    let start = Date::from_ymd_opt(today().year(), 1, 20).expect("January 20 exists");
    let dates = Selection::range(start, start + Duration::days(20));
    field(
        date_picker()
            .mode(Mode::Range)
            .width(Fill)
            .default_value(dates)
            .stacked(window_width < 768.0),
    )
    .label(text::label("Date Picker Range").line_height(1.375))
    .width(240)
    .into()
}
```
