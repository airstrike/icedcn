```rust
use icedcn::widget::calendar;

pub fn caption<Message: 'static>() -> calendar::Calendar<'static, Message> {
    calendar().bordered(true).caption_dropdown(true)
}
```
