```rust
use iced::{Fill, Pixels};
use icedcn::View;
use icedcn::widget::{column, row, separator, space, text};

fn list_item<'a>(term: &'a str, value: &'a str) -> View<'a, Message> {
    row![
        text::small(term).line_height(Pixels(20.0)),
        space::horizontal(),
        text::muted(value).line_height(Pixels(20.0)),
    ]
    .width(Fill)
    .into()
}

column![
    list_item("Item 1", "Value 1"),
    separator(),
    list_item("Item 2", "Value 2"),
    separator(),
    list_item("Item 3", "Value 3"),
]
.spacing(8)
.width(Fill)
.max_width(384.0)
```
