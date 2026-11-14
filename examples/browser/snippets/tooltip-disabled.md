```rust
use icedcn::widget::{button, tooltip};

tooltip(
    button("Disabled").style(button::Outline).disabled(true),
    "This feature is currently unavailable",
)
```
