```rust
use crate::icon;
use icedcn::Slot;
use icedcn::theme::text::muted;
use icedcn::widget::{field, input, input_group};

fn website<Message: Clone + 'static>() -> Slot<'static, Message> {
    let info = icon::info()
        .size(16)
        .line_height(1.0)
        .style(|theme| muted(theme).into());

    field(
        input_group(input("example.com"))
            .addon(input_group::addon(input_group::text("https://")))
            .addon(input_group::addon(info).align(input_group::Align::InlineEnd)),
    )
    .label("Website URL")
    .width(320)
    .into()
}
```
