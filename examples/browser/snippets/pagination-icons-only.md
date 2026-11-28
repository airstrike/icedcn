```rust
use crate::icon;
use icedcn::Horizontal;
use icedcn::widget::{field, pagination, row, select};

row![
    field(
        select(["10", "25", "50", "100"].map(|count| select::item(count, count)))
            .default_value("25")
            .icon(icon::chevron_down)
            .width(80)
    )
    .label("Rows per page")
    .orientation(Horizontal)
    .width(iced::Shrink),
    pagination([
        pagination::link(icon::chevron_left())
            .on_press(Message::Previous)
            .into(),
        pagination::link(icon::chevron_right())
            .on_press(Message::Next)
            .into(),
    ])
    .width(iced::Shrink)
]
.align_y(iced::Center)
.spacing(16)
```
