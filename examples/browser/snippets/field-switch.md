```rust
use icedcn::Horizontal;
use icedcn::Slot;
use icedcn::widget::{field, switch};

fn authentication<'a, M: Clone + 'a>() -> Slot<'a, M> {
    field(switch())
        .orientation(Horizontal)
        .control_last()
        .label("Multi-factor authentication")
        .into()
}
```
