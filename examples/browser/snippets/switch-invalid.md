```rust
use icedcn::theme::text::Destructive;
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, Slot};

fn terms<'a, M: Clone + 'a>() -> Slot<'a, M> {
    field(switch().style(switch::Invalid))
        .orientation(Horizontal)
        .control_last()
        .label(field::label("Accept terms and conditions").style(Destructive))
        .description("You must accept the terms and conditions to continue.")
        .width(384)
        .into()
}
```
