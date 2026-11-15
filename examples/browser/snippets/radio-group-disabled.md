```rust
use iced::Shrink;
use icedcn::theme;
use icedcn::theme::tweak::Alpha;
use icedcn::widget::{field, radio_group, text};
use icedcn::{Horizontal, Slot};

fn availability<M: Clone + 'static>() -> Slot<'static, M> {
    radio_group(|g| {
        field::group(["Disabled", "Option 2", "Option 3"].map(|name| {
            let disabled = name == "Disabled";
            let label = text::small(name)
                .style(theme::text::Foreground + Alpha(if disabled { 0.5 } else { 1. }));
            field(g.item(name))
                .orientation(Horizontal)
                .label(label)
                .disabled(disabled)
                .into()
        }))
        .spacing(8)
        .width(Shrink)
        .into()
    })
    .default_value("Option 2")
    .into()
}
```
