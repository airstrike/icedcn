```rust
use icedxdy::theme;
use icedxdy::widget::{field, text, textarea};

field(textarea("Type your message here.").style(textarea::Invalid))
    .label(text::label("Message").style(theme::text::Destructive))
    .description("Please enter a valid message.")
```
