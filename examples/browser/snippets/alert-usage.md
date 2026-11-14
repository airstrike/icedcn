## Usage

```rust
use iced::Widget;
use iced::widget::svg;
use iced::{Element, Subscription, Task, window};
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
        const ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/></svg>"#;
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
                        color: Some(theme.card.foreground.into()),
                    })
                    .boxed(),
            )
            .title("Heads up!")
            .description("You can add components and dependencies to your app using the cli.")
            .action(button("Enable").style(button::Outline))
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

Drop the `.icon(...)` call to render a title-and-description-only alert. Drop `.action(...)` for the simple callout. Use `.style(alert::Destructive)` for the destructive variant.

The action stays enabled without a handler, as in the source example. Add `.on_press(Message::Enable)` to the button to publish your application’s message; handle it in `update`. Alert itself has no open or checked state.
