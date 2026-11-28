## Controlled State

Use `.open(value)` and `.on_open_change(...)` to control the state. The message carries the requested new value; assign it in `update`. Without `.open(...)`, the widget owns its state. Use `.default_open(true)` to start it open.

```rust
use icedcn::View;
use icedcn::widget::{button, collapsible, column};

#[derive(Debug, Clone)]
enum Message {
    OpenChanged(bool),
}

fn update(open: &mut bool, message: Message) {
    match message {
        Message::OpenChanged(next) => *open = next,
    }
}

fn view<'a>(open: bool) -> View<'a, Message> {
    collapsible(|c| column![c.trigger(button("Toggle")), c.content("Content")].into())
        .open(open)
        .on_open_change(Message::OpenChanged)
        .into()
}
```
