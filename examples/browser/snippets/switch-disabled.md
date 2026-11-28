```rust
use icedcn::theme::{text::Foreground, tweak::Alpha};
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, View};

fn unavailable<'a, M: Clone + 'a>() -> View<'a, M> {
    field(switch())
        .orientation(Horizontal)
        .label(field::label("Disabled").style(Foreground + Alpha(0.5)))
        .disabled(true)
        .into()
}
```
