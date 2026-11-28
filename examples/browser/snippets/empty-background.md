```rust
use crate::icon;
use iced::Fill;
use icedcn::theme::container::Transparent;
use icedcn::theme::tweak::{Alpha, Bg, Token};
use icedcn::widget::{button, container, empty};

container(
    empty::empty()
        .media(empty::media(icon::bell()).style(empty::Icon))
        .title("No Notifications")
        .description("You're all caught up. New notifications will appear here.")
        .content(button![icon::refresh_ccw(), "Refresh"].style(button::Outline)),
)
.width(Fill)
.style(Transparent + Bg(Token::Muted) + Alpha(0.3))
```
