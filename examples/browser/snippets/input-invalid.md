```rust
use icedcn::theme;
use icedcn::widget::{field, input, text};

field(
    input("Error")
        .width(Fill)
        .style(theme::text_input::Invalid),
)
.label(text::label("Invalid Input").style(theme::text::Destructive))
.description("This field contains validation errors.")
.width(320)
```
