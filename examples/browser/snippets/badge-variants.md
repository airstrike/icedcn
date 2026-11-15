```rust
use icedcn::widget::{badge, row};

fn example<'a>() -> row::Row<'a, ()> {
    row![
        badge("Default"),
        badge("Secondary").style(badge::Secondary),
        badge("Destructive").style(badge::Destructive),
        badge("Outline").style(badge::Outline),
        badge("Ghost").style(badge::Ghost),
    ]
    .spacing(8)
    .wrap()
}
```
