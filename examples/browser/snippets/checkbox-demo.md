```rust
use iced::{Fill, Pixels};
use icedcn::theme;
use icedcn::theme::tweak::Alpha;
use icedcn::widget::{Slot, checkbox, column, container, field, text};

column![
    field(checkbox())
        .orientation(field::Horizontal)
        .label(Slot::from(
            container(text::label("Accept terms and conditions")).height(14)
        )),
    field(container(checkbox().default_checked(true)).padding([1, 0, 0, 0]))
        .orientation(field::Horizontal)
        .label(text::label("Accept terms and conditions").line_height(Pixels(19.25)))
        .description(
            text::muted("By clicking this checkbox, you agree to the terms.")
                .line_height(Pixels(21.0))
        ),
    field(checkbox().disabled(true))
        .orientation(field::Horizontal)
        .disabled(true)
        .label(
            text::label("Enable notifications")
                .line_height(Pixels(19.25))
                .style(theme::text::Foreground + Alpha(0.5))
        ),
    field(container(checkbox()).padding([1, 0, 0, 0]))
        .orientation(field::Horizontal)
        .label(text::label("Enable notifications").line_height(Pixels(20.0)))
        .description(
            text::muted("You can enable or disable notifications at any time.")
                .line_height(Pixels(21.0))
        )
        .card(),
]
.spacing(20)
.width(Fill)
.max_width(384.0)
```
