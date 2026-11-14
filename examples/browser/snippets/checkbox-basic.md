```rust
use iced::Pixels;
use icedcn::widget::{checkbox, field, text};

field(checkbox())
    .orientation(field::Horizontal)
    .label(text::label("Accept terms and conditions").line_height(Pixels(19.25)))
    .width(224)
```
