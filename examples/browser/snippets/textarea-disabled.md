```rust
use icedcn::View;
use icedcn::theme;
use icedcn::theme::alpha;
use icedcn::widget::{field, text, textarea};

pub fn view<'a>() -> View<'a> {
    field(textarea("Type your message here."))
        .disabled(true)
        .label(text::label("Message").style(theme::text::Foreground + alpha(0.5)))
        .into()
}
```
