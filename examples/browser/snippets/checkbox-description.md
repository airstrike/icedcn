```rust
use iced::Pixels;
use icedxdy::widget::{checkbox, container, field, text};

field(container(checkbox().default_checked(true)).padding([1, 0, 0, 0]))
    .orientation(field::Horizontal)
    .label(text::label("Accept terms and conditions").line_height(Pixels(19.25)))
    .description(
        text::muted("By clicking this checkbox, you agree to the terms and conditions.")
            .line_height(Pixels(21.0)),
    )
    .width(288)
```
