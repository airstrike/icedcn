```rust
use icedxdy::theme::tweak::Alpha;
use icedxdy::theme;
use icedxdy::widget::{field, text, textarea};

field(textarea("Type your message here.")).disabled(true)
    .label(text::label("Message").style(theme::text::Foreground + Alpha(0.5)))
```
