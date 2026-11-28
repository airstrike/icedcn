```rust
use iced::Fill;
use icedcn::theme;
use icedcn::widget::{View, field, input, row};

field(input("This field is required").width(Fill))
    .label(View::from(
        row![
            field::label("Required Field"),
            field::label("*").style(theme::text::Destructive),
        ]
        .spacing(8),
    ))
    .description("This field must be filled out.")
    .width(320)
```
