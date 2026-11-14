```rust
use icedxdy::widget::{button, row, spinner};

row![
    button![spinner().circle(), "Generating"]
        .style(button::Outline)
        .icon_start()
        .disabled(true),
    button!["Downloading", spinner().circle()]
        .style(button::Secondary)
        .icon_start()
        .disabled(true),
]
.spacing(8)
```
