```rust
use iced::Pixels;
use icedxdy::theme;
use icedxdy::widget::{checkbox, field, text};

field(checkbox().style(theme::checkbox::Invalid))
    .orientation(field::Horizontal)
    .label(
        text::label("Accept terms and conditions")
            .line_height(Pixels(19.25))
            .style(theme::text::Destructive),
    )
    .width(224)
```
