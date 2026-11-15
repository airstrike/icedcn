## Usage

Use a slide count and a factory to compose the track and its previous/next controls. All slides stay mounted as the track moves.

```rust
use icedcn::Slot;
use icedcn::widget::{carousel, container, text};

pub fn view<'a, Message: Clone + 'static>() -> Slot<'a, Message> {
    carousel(5, |index| {
        container(text::p(format!("Slide {}", index + 1)))
            .padding(24)
            .height(160)
    })
    .width(320)
    .into()
}
```

Responsive examples take the actual window width. Obtain its initial value and subsequent changes through iced window tasks and subscriptions; do not substitute the preview's available content width.

```rust
use iced::{Subscription, Task, window};

#[derive(Debug, Clone)]
pub enum Message {
    Width(f32),
}

pub struct Window {
    pub width: f32,
}

impl Window {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self { width: 1024.0 },
            window::oldest()
                .and_then(window::size)
                .map(|size| Message::Width(size.width)),
        )
    }

    pub fn update(&mut self, Message::Width(width): Message) {
        self.width = width;
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|(_, size)| Message::Width(size.width))
    }
}
```
