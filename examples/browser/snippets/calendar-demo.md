```rust
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Selection, today};

pub fn demo<Message: 'static>() -> calendar::Calendar<'static, Message> {
    calendar()
        .bordered(true)
        .default_selected(Selection::single(today()))
        .caption_dropdown(true)
}
```
