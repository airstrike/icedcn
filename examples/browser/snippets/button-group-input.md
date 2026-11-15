```rust
use crate::icon;
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{button, button_group, input};

pub fn view<'a>() -> Slot<'a, ()> {
    button_group![
        input("Search...").width(Fill),
        button(icon::search().size(16).line_height(1.0)).style(button::Outline)
    ]
    .width(Fill.max(240.))
    .into()
}
```
