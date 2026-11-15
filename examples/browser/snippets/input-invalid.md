```rust
use iced::Fill;
use icedcn::theme;
use icedcn::widget::{field, input};

field(input("Error").width(Fill).style(theme::text_input::Invalid))
    .label(field::label("Invalid Input").style(theme::text::Destructive))
    .description("This field contains validation errors.")
    .width(320)
```
