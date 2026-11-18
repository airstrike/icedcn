```rust
use icedcn::theme::{text::Foreground, tweak::Alpha};
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, Slot};

fn unavailable<'a, M: Clone + 'a>() -> Slot<'a, M> {
    field(switch())
        .orientation(Horizontal)
        .label(field::label("Disabled").style(Foreground + Alpha(0.5)))
        .disabled(true)
        .into()
}
```
