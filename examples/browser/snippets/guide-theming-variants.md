## Variants

A variant is a unit struct named after the look it selects. Each widget module re-exports its own:

```rust
use icedcn::View;
use icedcn::widget::{button, row};

pub fn actions<'a>() -> View<'a> {
    row![
        button("Save"),
        button("Cancel").style(button::Outline),
        button("Later").style(button::Secondary),
        button("Docs").style(button::Ghost),
        button("Delete").style(button::Destructive),
        button("Learn more").style(button::Link),
    ]
    .spacing(8)
    .into()
}
```

Shared variants are one type. `Destructive` selects the destructive look of every component whose style converts from it: buttons, badges, alerts, alert dialog media, code blocks, containers, menu items, and text. Items and keyboard keys take a container's style, so `Destructive` gives them the container's destructive look. A component whose style has no destructive look has no such conversion, so passing it there is a compile error rather than a silent fallback.

Choose the variant that matches the action's meaning before overriding a color. Each component page lists the variants it supports.
