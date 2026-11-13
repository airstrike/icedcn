```rust
use icedxdy::theme;
use icedxdy::theme::tweak::Alpha;
use icedxdy::widget::{checkbox, field, text};

field(checkbox().disabled(true))
    .orientation(field::Horizontal)
    .label(text::label("Enable notifications").style(theme::text::Foreground + Alpha(0.5)))
    .width(224)
```
