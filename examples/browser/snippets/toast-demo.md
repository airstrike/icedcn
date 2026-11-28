```rust
use icedcn::{
    View,
    widget::{button, toast},
};

#[derive(Clone)]
enum Message {
    Undo,
}

fn show_toast<'a>() -> View<'a, Message> {
    toast::trigger(
        button("Show Toast").style(button::Outline),
        toast::toast("Event has been created")
            .description("Sunday, December 03, 2023 at 9:00 AM")
            .action("Undo", Message::Undo),
    )
}
```
