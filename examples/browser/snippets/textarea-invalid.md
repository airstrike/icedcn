```rust
use icedcn::theme;
use icedcn::widget::{field, text, textarea};

field(textarea("Type your message here.").style(textarea::Invalid))
    .label(text::label("Message").style(theme::text::Destructive))
    .description("Please enter a valid message.")
```
