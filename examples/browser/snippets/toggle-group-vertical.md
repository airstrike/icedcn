```rust
use icedcn::Vertical;
use icedcn::widget::toggle_group;

toggle_group(formatting)
    .mode(toggle_group::Mode::Multiple)
    .default_value(["bold", "italic"])
    .orientation(Vertical)
    .spacing(4)
```
