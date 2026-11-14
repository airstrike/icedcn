```rust
use iced::{Element, Subscription, Task, window};
use icedxdy::Size;
use icedxdy::Theme;
use icedxdy::widget::text::Wrapping;
use icedxdy::widget::{alert, button};

struct Example {
    window_width: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Width(f32),
}

impl Example {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                window_width: 1024.0,
            },
            window::oldest()
                .and_then(window::size)
                .map(|size| Message::Width(size.width)),
        )
    }

    fn update(&mut self, Message::Width(width): Message) {
        self.window_width = width;
    }

    fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|(_, size)| Message::Width(size.width))
    }

    fn view(&self) -> Element<'static, Message, Theme> {
        let wrapping = if self.window_width < 768.0 {
            Wrapping::Balanced
        } else {
            Wrapping::Word
        };
        alert()
            .description_wrapping(wrapping)
            .title("Dark mode is now available")
            .description("Enable it under your profile settings to get started.")
            .action(button("Enable").size(Size::Xs))
            .max_width(448)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(Example::new, Example::update, Example::view)
        .subscription(Example::subscription)
        .theme(|_: &Example| Theme::light())
        .run()
}
```
