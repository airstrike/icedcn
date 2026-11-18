```rust
use icedcn::widget::calendar;

pub fn caption<'a, Message: 'a>() -> calendar::Calendar<'a, Message> {
    calendar().bordered(true).caption_dropdown(true)
}
```
