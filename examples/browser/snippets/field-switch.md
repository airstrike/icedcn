```rust
use icedcn::Horizontal;
use icedcn::View;
use icedcn::widget::{field, switch};

fn authentication<'a, M: Clone + 'a>() -> View<'a, M> {
    field(switch())
        .orientation(Horizontal)
        .control_last()
        .label("Multi-factor authentication")
        .into()
}
```
