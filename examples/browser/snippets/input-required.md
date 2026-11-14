```rust
use iced::Fill;
use icedcn::theme;
use icedcn::widget::{Slot, field, input, row};

field(input("This field is required").width(Fill))
    .label(Slot::from(row![
        field::label("Required Field"),
        field::label("*").style(theme::text::Destructive),
    ].spacing(8)))
    .description("This field must be filled out.")
    .width(320)
```
