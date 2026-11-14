```rust
use iced::Fill;
use iced::Widget;
use iced::widget::svg;
use iced::{Element, Subscription, Task, window};
use icedcn::Theme;
use icedcn::widget::text::Wrapping;
use icedcn::widget::{alert, column};

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
        const CHECK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#;
        const INFO: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/></svg>"#;
        let wrapping = if self.window_width < 768.0 {
            Wrapping::Balanced
        } else {
            Wrapping::Word
        };
        column![
            alert()
                .description_wrapping(wrapping)
                .icon(
                    svg(svg::Handle::from_memory(CHECK.as_bytes()))
                        .width(16)
                        .height(16)
                        .style(|theme: &Theme, _| svg::Style {
                            color: Some(theme.card.foreground.into()),
                        })
                        .boxed(),
                )
                .title("Payment successful")
                .description(
                    "Your payment of $29.99 has been processed. A receipt has been sent to your email address.",
                )
                .max_width(448),
            alert()
                .description_wrapping(wrapping)
                .icon(
                    svg(svg::Handle::from_memory(INFO.as_bytes()))
                        .width(16)
                        .height(16)
                        .style(|theme: &Theme, _| svg::Style {
                            color: Some(theme.card.foreground.into()),
                        })
                        .boxed(),
                )
                .title("New feature available")
                .description(
                    "We've added dark mode support. You can enable it in your account settings.",
                )
                .max_width(448),
        ]
        .spacing(16)
        .width(Fill.max(448))
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
