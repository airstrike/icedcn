```rust
use icedcn::widget::{badge, row, spinner};

row![
    badge![spinner().size(12.0), "Syncing"].icon_start(),
    badge![spinner().size(12.0), "Updating"]
        .icon_start()
        .style(badge::Secondary),
    badge![spinner().size(12.0), "Processing"]
        .icon_start()
        .style(badge::Outline),
]
.spacing(16)
.align_y(Center)
```
