```rust
use iced::Fill;
use icedcn::{Slot, widget::{button, button_group, field, input}};

pub fn view() -> Slot<'static, ()> {
    field(button_group![
        input("Type to search...").width(Fill),
        button("Search").style(button::Outline),
    ].width(Fill)).label("Search").width(320).into()
}
```
