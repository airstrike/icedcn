```rust
use icedcn::widget::{row, toggle};

row![
    toggle("Disabled").disabled(true),
    toggle("Disabled").style(toggle::Outline).disabled(true),
]
.spacing(8)
```
