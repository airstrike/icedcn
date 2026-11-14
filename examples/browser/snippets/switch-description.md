```rust
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, Slot};

fn sharing<M: Clone + 'static>() -> Slot<'static, M> {
    field(switch())
        .orientation(Horizontal)
        .control_last()
        .label("Share across devices")
        .description("Focus is shared across devices, and turns off when you leave the app.")
        .width(384)
        .into()
}
```
