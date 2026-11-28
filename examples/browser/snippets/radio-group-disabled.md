```rust
use iced::Shrink;
use icedcn::theme;
use icedcn::theme::alpha;
use icedcn::widget::{field, radio_group, text};
use icedcn::{Horizontal, View};

fn availability<'a, M: Clone + 'static>() -> View<'a, M> {
    radio_group(|g| {
        field::group(["Disabled", "Option 2", "Option 3"].map(|name| {
            let disabled = name == "Disabled";
            let label = text::small(name)
                .style(theme::text::Foreground + alpha(if disabled { 0.5 } else { 1. }));
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
