```rust
use icedcn::widget::{badge, row, spinner};

fn example<'a>() -> row::Row<'a, ()> {
    row![
        badge![spinner().circle().size(12.0), "Deleting"]
            .icon_start()
            .style(badge::Destructive),
        badge!["Generating", spinner().circle().size(12.0)]
            .icon_end()
            .style(badge::Secondary),
    ]
    .spacing(8)
    .wrap()
}
```
