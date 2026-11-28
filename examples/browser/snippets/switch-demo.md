```rust
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, View};

fn airplane_mode<'a, M: Clone + 'a>() -> View<'a, M> {
    field(switch())
        .orientation(Horizontal)
        .label("Airplane Mode")
        .into()
}
```
