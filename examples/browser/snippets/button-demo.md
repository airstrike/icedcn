```rust
use crate::icon;
use icedcn::widget::{button, row};

row![
    button("Button").style(button::Outline),
    button(icon::arrow_up().size(16)).square().style(button::Outline),
]
.spacing(8)
```
