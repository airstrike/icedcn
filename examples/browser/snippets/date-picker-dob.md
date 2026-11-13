```rust
field(date_picker().caption_dropdown(true).placeholder("Select date").label(|value|row![text::small(value.map(|s|s.start.format("%-m/%-d/%Y").to_string()).unwrap_or_else(||"Select date".into())), space::horizontal(),icon::chevron_down()].into())).label("Date of birth")
```
