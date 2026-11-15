```rust
use iced::Fill;
use icedcn::widget::{date_picker, field, text};

field(
    date_picker()
        .width(Fill)
        .chevron(false)
        .placeholder("Select date")
        .format("%-m/%-d/%Y")
        .caption_dropdown(true),
)
.label(text::label("Date of birth").line_height(1.375))
.width(176)
```
