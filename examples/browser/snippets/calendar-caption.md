```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::calendar;

pub fn caption<Message: 'static>() -> Element<'static, Message, Theme> {
    calendar().bordered(true).caption_dropdown(true).into()
}
```
