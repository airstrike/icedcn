```rust
use iced::Fill;
use icedcn::widget::{button, field, input};

field(
    button::group![
        input("Type to search...").width(Fill),
        button("Search").style(button::Outline)
    ]
    .width(Fill),
)
.label("Search")
.width(320)
```
