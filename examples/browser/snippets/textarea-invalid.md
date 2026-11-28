```rust
use icedcn::View;
use icedcn::theme;
use icedcn::widget::{field, text, textarea};

pub fn view<'a>() -> View<'a> {
    field(textarea("Type your message here.").style(textarea::Invalid))
        .label(text::label("Message").style(theme::text::Destructive))
        .description("Please enter a valid message.")
        .into()
}
```
