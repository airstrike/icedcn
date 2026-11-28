```rust
use iced::Fill;
use icedcn::View;
use icedcn::widget::{date_picker, field, input, row, text};

pub fn view<'a, M: Clone + 'static>(window_width: f32) -> View<'a, M> {
    let compact = window_width < 768.0;
    row![
        field(
            date_picker()
                .width(Fill)
                .placeholder("Select date")
                .caption_dropdown(true)
        )
        .label(text::label("Date").line_height(1.375))
        .width(Fill),
        field(
            input("HH:mm:ss")
                .default_value("10:30:00")
                .text_size(if compact { 16.0 } else { 14.0 })
        )
        .label(text::label("Time").line_height(1.375))
        .width(if compact { 99 } else { 90 }),
    ]
    .spacing(20)
    .width(320)
    .into()
}
```
