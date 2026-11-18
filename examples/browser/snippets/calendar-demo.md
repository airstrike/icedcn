```rust
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Selection, today};

pub fn demo<'a, Message: 'a>() -> calendar::Calendar<'a, Message> {
    calendar()
        .bordered(true)
        .default_selected(Selection::single(today()))
        .caption_dropdown(true)
}
```
