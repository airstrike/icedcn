```rust
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, View};

fn sizes<'a, M: Clone + 'a>() -> View<'a, M> {
    field::group([
        field(switch().size(switch::Size::Sm))
            .orientation(Horizontal)
            .label("Small")
            .into(),
        field(switch())
            .orientation(Horizontal)
            .label("Default")
            .into(),
    ])
    .width(160)
    .into()
}
```
