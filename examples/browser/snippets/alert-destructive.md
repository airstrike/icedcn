```rust
use iced::Widget;
use iced::widget::svg;
use iced::{Element, Subscription, Task, window};
use icedcn::Theme;
use icedcn::widget::alert;
use icedcn::widget::text::Wrapping;

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
        const ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 8v4M12 16h.01"/></svg>"#;
        let wrapping = if self.window_width < 768.0 {
            Wrapping::Balanced
        } else {
            Wrapping::Word
        };
        alert()
            .description_wrapping(wrapping)
            .icon(
                svg(svg::Handle::from_memory(ICON.as_bytes()))
                    .width(16)
                    .height(16)
                    .style(|theme: &Theme, _| svg::Style {
                        color: Some(theme.destructive.base.into()),
                    })
                    .boxed(),
            )
            .title("Payment failed")
            .description(
                "Your payment could not be processed. Please check your payment method and try again.",
            )
            .style(alert::Destructive)
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
