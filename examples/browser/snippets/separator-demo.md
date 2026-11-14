```rust
use iced::{Fill, Pixels};
use icedxdy::widget::{column, separator, text};

column![
    column![
        text::label("shadcn/ui").height(14),
        text::muted("The Foundation for your Design System").line_height(Pixels(20.0)),
    ]
    .spacing(6),
    separator(),
    text::small("A set of beautifully designed components that you can customize, extend, and build on.")
        .line_height(Pixels(20.0)),
]
.spacing(16)
.width(Fill)
.max_width(384.0)
```
