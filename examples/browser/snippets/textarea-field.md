```rust
use icedcn::Slot;
use icedcn::widget::{field, textarea};

pub fn view() -> Slot<'static, ()> {
    field(textarea("Type your message here."))
        .label("Message")
        .description("Enter your message below.")
        .description_above()
        .into()
}
```
