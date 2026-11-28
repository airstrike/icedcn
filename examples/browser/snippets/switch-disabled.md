```rust
use icedcn::theme::{alpha, text::Foreground};
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, View};

fn unavailable<'a, M: Clone + 'a>() -> View<'a, M> {
    field(switch())
        .orientation(Horizontal)
        .label(field::label("Disabled").style(Foreground + alpha(0.5)))
        .disabled(true)
        .into()
}
```
