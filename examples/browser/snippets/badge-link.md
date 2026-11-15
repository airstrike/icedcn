```rust
use crate::icon;
use iced::Task;
use iced::widget::Id;
use icedcn::widget::badge;
use icedcn::widget::operation::scroll;

#[derive(Clone)]
enum Message {
    OpenLink,
}

fn example() -> badge::Badge<'static, Message> {
    badge![
        "Open Link",
        icon::arrow_up_right().size(12).line_height(1.0)
    ]
    .icon_end()
    .on_press(Message::OpenLink)
}

fn update(message: Message, scrollable: Id, link_section: Id) -> Task<Message> {
    match message {
        Message::OpenLink => scroll::scroll_into_view(scrollable, link_section),
    }
}
```

Handle `Message::OpenLink` in your application update. Pass the IDs of your scrollable and Link section to `update`; it returns the public scroll operation that navigates to that section. The browser example uses the same operation through its page message.
