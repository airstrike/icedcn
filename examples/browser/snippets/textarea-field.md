```rust
use icedcn::View;
use icedcn::widget::{field, textarea};

pub fn view<'a>() -> View<'a> {
    field(textarea("Type your message here."))
        .label("Message")
        .description("Enter your message below.")
        .description_above()
        .into()
}
```
