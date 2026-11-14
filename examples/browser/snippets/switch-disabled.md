```rust
use icedcn::theme::{text::Foreground, tweak::Alpha};
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, Slot};

fn unavailable<M: Clone + 'static>() -> Slot<'static, M> {
    field(switch())
        .orientation(Horizontal)
        .label(field::label("Disabled").style(Foreground + Alpha(0.5)))
        .disabled(true)
        .into()
}
```
