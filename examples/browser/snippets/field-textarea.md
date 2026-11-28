```rust
use icedcn::View;
use icedcn::widget::{field, textarea};

fn feedback<'a, M: Clone + 'a>() -> View<'a, M> {
    field(textarea("Your feedback helps us improve..."))
        .label("Feedback")
        .description("Share your thoughts about our service.")
        .width(320)
        .into()
}
```
