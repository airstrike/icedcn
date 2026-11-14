```rust
use icedcn::theme::tweak::Alpha;
use icedcn::theme;
use icedcn::widget::{field, text, textarea};

field(textarea("Type your message here.")).disabled(true)
    .label(text::label("Message").style(theme::text::Foreground + Alpha(0.5)))
```
