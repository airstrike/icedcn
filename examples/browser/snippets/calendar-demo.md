```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Selection, today};

pub fn demo<Message: 'static>() -> Element<'static, Message, Theme> {
    calendar()
        .bordered(true)
        .default_selected(Selection::single(today()))
        .caption_dropdown(true)
        .into()
}
```
