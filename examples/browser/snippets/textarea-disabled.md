```rust
use icedcn::Slot;
use icedcn::theme;
use icedcn::theme::tweak::Alpha;
use icedcn::widget::{field, text, textarea};

pub fn view<'a>() -> Slot<'a, ()> {
    field(textarea("Type your message here."))
        .disabled(true)
        .label(text::label("Message").style(theme::text::Foreground + Alpha(0.5)))
        .into()
}
```
