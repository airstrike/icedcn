```rust
use icedcn::Horizontal;
use icedcn::Slot;
use icedcn::widget::{field, switch};

fn authentication<M: Clone + 'static>() -> Slot<'static, M> {
    field(switch())
        .orientation(Horizontal)
        .control_last()
        .label("Multi-factor authentication")
        .into()
}
```
