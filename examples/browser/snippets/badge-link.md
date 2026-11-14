```rust
use iced::{Element, Task, Widget};
use iced::widget::{svg, Id};
use icedxdy::Theme;
use icedxdy::widget::badge;
use icedxdy::widget::operation::scroll;

#[derive(Clone)]
enum Message { OpenLink }

fn example() -> Element<'static, Message, Theme> {
    badge!["Open Link", svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 7h10v10M7 17 17 7"/></svg>"#.as_bytes()))
                .width(12)
                .height(12)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.primary.foreground.into()) })
                .boxed()]
        .icon_end()
        .on_press(Message::OpenLink)
        .into()
}

fn update(message: Message, scrollable: Id, link_section: Id) -> Task<Message> {
    match message {
        Message::OpenLink => scroll::scroll_into_view(scrollable, link_section),
    }
}
```

Handle `Message::OpenLink` in your application update. Pass the IDs of your scrollable and Link section to `update`; it returns the public scroll operation that navigates to that section. The browser example uses the same operation through its page message.
