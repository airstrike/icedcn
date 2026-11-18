```rust
use icedcn::widget::{button, column, spinner};

column![
    button![spinner(), "Loading..."]
        .size(Size::Sm)
        .icon_start()
        .disabled(true),
    button![spinner(), "Please wait"]
        .size(Size::Sm)
        .icon_start()
        .style(button::Outline)
        .disabled(true),
    button![spinner(), "Processing"]
        .size(Size::Sm)
        .icon_start()
        .style(button::Secondary)
        .disabled(true),
]
.spacing(16)
.align_x(Center)
```
