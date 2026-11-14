```rust
use iced::Fill;
use icedcn::widget::{button, input, row};

row![
    input("Search...").width(Fill),
    button("Search"),
]
.spacing(8)
.width(320)
```
