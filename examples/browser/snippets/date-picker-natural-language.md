```rust
use icedcn::View;
use icedcn::widget::calendar::Date;
use icedcn::widget::{date_picker, field, text};

pub fn view<'a>(date: Option<Date>, window_width: f32) -> View<'a, Option<Date>> {
    let display = date
        .map(|d| d.format("%B %d, %Y").to_string())
        .unwrap_or_default();
    field(
        date_picker::input("Tomorrow or next week")
            .default_text("In 2 days")
            .parse_with(date_picker::relative)
            .caption_dropdown(true)
            .text_size(if window_width < 768.0 { 16.0 } else { 14.0 })
            .value(date)
            .on_value_change(|next| next),
    )
    .label(text::label("Schedule Date").line_height(1.375))
    .description(format!("Your post will be published on {display}."))
    .width(320)
    .into()
}
```

Initialize the date with `date_picker::relative("In 2 days", calendar::today())` and store the value this view publishes.
