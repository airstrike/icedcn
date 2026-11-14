```rust
use icedcn::rich_text;
use icedcn::theme::tweak::Token;
use icedcn::widget::rich_text::span;
use icedcn::widget::{field, input, text, Slot};

field(
    input("This field is required")
        .width(Fill),
)
.label(Slot::from(rich_text![
    span("Required Field ").size(text::size::SM).medium(),
    span("*").size(text::size::SM).color(Token::Destructive),
]))
.description("This field must be filled out.")
.width(320)
```
