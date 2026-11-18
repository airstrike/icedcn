```rust
use icedcn::widget::{row, toggle};

row![
    toggle![icon::italic(), "Italic"].style(toggle::Outline),
    toggle![icon::bold(), "Bold"].style(toggle::Outline),
]
.spacing(8)
```
