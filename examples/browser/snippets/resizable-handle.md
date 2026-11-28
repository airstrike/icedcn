```rust
use icedcn::widget::resizable;

frame(
    resizable(panel("Sidebar"), panel("Content"))
        .default_value(0.25)
        .with_handle(),
    450.0,
)
```
