Give the viewport an ID in `view`.

```rust
scroller(rows).id("conversation")
```

Return the scroll task from `update` when the user chooses a message.

```rust
use icedxdy::widget::scroller::{self, Animation, Target};

#[derive(Debug, Clone)]
enum Message {
    Jump(u64),
}

fn update(message: Message) -> iced::Task<Message> {
    match message {
        Message::Jump(key) => {
            scroller::scroll_to("conversation", Target::Item(key), Animation::Smooth)
        }
    }
}
```
