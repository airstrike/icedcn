```rust
use icedcn::Slot;
use icedcn::widget::{field, textarea};

pub fn view<'a>() -> Slot<'a, ()> {
    field(textarea("Type your message here."))
        .label("Message")
        .description("Enter your message below.")
        .description_above()
        .into()
}
```
