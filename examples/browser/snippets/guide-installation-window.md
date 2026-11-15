## Your first window

The CLI creates a starter application for you. If you followed the manual setup, put this in `src/main.rs`:

```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::{button, column, text};

#[derive(Default)]
struct Counter {
    count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        column![
            text(format!("Count: {}", self.count)),
            button("Increment").on_press(Message::Increment),
        ]
        .spacing(12)
        .padding(24)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("My icedcn application")
        .run()
}
```

Check the application, then open the window:

```sh
cargo check
cargo run --locked
```

Commit the generated `Cargo.lock` with your application. This example follows the system appearance and uses the system font. See [Typography](icedcn://component/typography) for text helpers and [Icons](icedcn://guide/installation#icons) for the generated icon module used by other examples.
