```rust
use icedcn::widget::calendar;

pub fn basic<Message: 'static>() -> calendar::Calendar<'static, Message> {
    calendar().bordered(true)
}
```
