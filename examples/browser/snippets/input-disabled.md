```rust
use iced::Fill;
use icedcn::widget::{field, input};
use icedcn::theme;
use icedcn::theme::tweak::Alpha;

field(input("Email").width(Fill))
    .label(field::label("Email").style(theme::text::Foreground + Alpha(0.5)))
    .description("This field is currently disabled.")
    .disabled(true)
    .width(320)
```
