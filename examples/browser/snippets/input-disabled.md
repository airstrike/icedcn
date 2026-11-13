```rust
use icedxdy::theme;
use icedxdy::theme::tweak::Alpha;
use icedxdy::widget::{field, input, text};

field(input("Email").width(Fill).disabled(true))
    .label(text::label("Email").style(theme::text::Foreground + Alpha(0.5)))
    .description("This field is currently disabled.")
    .width(320)
```
