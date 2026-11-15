```rust
use iced::Shrink;
use icedcn::widget::{field, radio_group};
use icedcn::{Horizontal, Slot};

fn described<M: Clone + 'static>() -> Slot<'static, M> {
    let choices = [
        ("Default", "Standard spacing for most use cases."),
        ("Comfortable", "More space between elements."),
        ("Compact", "Minimal spacing for dense layouts."),
    ];
    radio_group(move |g| {
        field::group(choices.map(|(name, detail)| {
            field(g.item(name))
                .orientation(Horizontal)
                .label(name)
                .description(detail)
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
