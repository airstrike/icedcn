```rust
use iced::Fill;
use icedcn::View;
use icedcn::widget::{button, column, textarea};

pub fn view<'a>() -> View<'a> {
    column![
        textarea("Type your message here."),
        button("Send message").width(Fill),
    ]
    .spacing(8)
    .width(Fill)
    .into()
}
```
