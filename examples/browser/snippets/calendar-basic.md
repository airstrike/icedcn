```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::calendar;

pub fn basic<Message: 'static>() -> Element<'static, Message, Theme> {
    calendar().bordered(true).into()
}
```
