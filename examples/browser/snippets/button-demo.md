```rust
use icedxdy::widget::{button, row};
use crate::icon;

row![
    button("Button").style(button::Outline),
    button(icon::arrow_up())
        .square()
        .style(button::Outline),
]
.spacing(8)
```
