```rust
use icedxdy::widget::{scroller, text};
use scroller::{Position, Target};

scroller(rows)
    .id("conversation")
    .default_scroll_position(Position::Start)
    .on_reader_change(Message::Reader)
    .end_button(reader.can_scroll_down, |size| text("↓").size(size), Message::Jump(Target::End))
```

Store the latest `Reader` in `update`. Its `can_scroll_up` and `can_scroll_down` fields drive the footer. Handle `Message::Jump(target)` by returning `scroller::scroll_to("conversation", target, scroller::Animation::Smooth)`.
