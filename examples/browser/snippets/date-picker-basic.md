```rust
use iced::Fill;
use icedcn::widget::{date_picker, field, text};

field(
    date_picker()
        .width(Fill)
        .chevron(false)
        .close_on_select(false),
)
.label(text::label("Date").line_height(1.375))
.width(176)
```
