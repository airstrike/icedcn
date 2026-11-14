```rust
use iced::Fill;
use icedcn::widget::{button, button_group, field, input};

field(
    button_group![
        input("Type to search...").width(Fill),
        button("Search").style(button::Outline)
    ]
    .width(Fill),
)
.label("Search")
.width(320)
```
