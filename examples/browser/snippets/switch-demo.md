```rust
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, Slot};

fn airplane_mode<M: Clone + 'static>() -> Slot<'static, M> {
    field(switch())
        .orientation(Horizontal)
        .label("Airplane Mode")
        .into()
}
```
