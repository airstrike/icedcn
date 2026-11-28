```rust
use iced::Shrink;
use icedcn::widget::{field, radio_group, text};
use icedcn::{Horizontal, View};

fn density<'a, M: Clone + 'static>() -> View<'a, M> {
    radio_group(|g| {
        field::group(["Default", "Comfortable", "Compact"].map(|name| {
            field(g.item(name))
                .orientation(Horizontal)
                .label(text::label(name))
                .into()
        }))
        .spacing(8)
        .width(Shrink)
        .into()
    })
    .default_value("Comfortable")
    .into()
}
```
