```rust
use icedcn::Slot;
use icedcn::widget::calendar::Date;
use icedcn::widget::{date_picker, field, text};

pub fn view<M: Clone + 'static>(window_width: f32) -> Slot<'static, M> {
    let start = Date::from_ymd_opt(2025, 6, 1).expect("valid example date");
    field(
        date_picker::input("June 01, 2025").default_value(start)
            .text_size(if window_width < 768.0 { 16.0 } else { 14.0 }),
    )
    .label(text::label("Subscription Date").line_height(1.375))
    .width(192)
    .into()
}
```
