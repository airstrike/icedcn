```rust
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, Slot};

fn airplane_mode<'a, M: Clone + 'a>() -> Slot<'a, M> {
    field(switch())
        .orientation(Horizontal)
        .label("Airplane Mode")
        .into()
}
```
