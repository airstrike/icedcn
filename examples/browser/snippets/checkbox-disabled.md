```rust
use iced::Pixels;
use icedxdy::theme;
use icedxdy::theme::tweak::Alpha;
use icedxdy::widget::{checkbox, field, text};

field(checkbox().disabled(true))
    .orientation(field::Horizontal)
    .disabled(true)
    .label(
        text::label("Enable notifications")
            .line_height(Pixels(19.25))
            .style(theme::text::Foreground + Alpha(0.5)),
    )
    .width(224)
```
