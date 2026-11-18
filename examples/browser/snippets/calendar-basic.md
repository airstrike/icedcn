```rust
use icedcn::widget::calendar;

pub fn basic<'a, Message: 'a>() -> calendar::Calendar<'a, Message> {
    calendar().bordered(true)
}
```
