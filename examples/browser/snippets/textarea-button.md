```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{button, column, textarea};

pub fn view<'a>() -> Slot<'a, ()> {
    column![
        textarea("Type your message here."),
        button("Send message").width(Fill),
    ]
    .spacing(8)
    .width(Fill)
    .into()
}
```
