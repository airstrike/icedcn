```rust
use crate::icon;
use icedcn::widget::{badge, row};

fn example<'a>() -> row::Row<'a, ()> {
    row![
        badge![icon::badge_check().size(12).line_height(1.0), "Verified"]
            .icon_start()
            .style(badge::Secondary),
        badge!["Bookmark", icon::bookmark().size(12).line_height(1.0)]
            .icon_end()
            .style(badge::Outline),
    ]
    .spacing(8)
    .wrap()
}
```
