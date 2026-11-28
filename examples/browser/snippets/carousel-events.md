## Events

`.on_value_change(Message::Selected)` publishes the requested zero-based snap index. Store it in `update` and pass it back to `.value(...)` to control the carousel.

```rust
use icedcn::View;
use icedcn::widget::{carousel, container, text};

#[derive(Debug, Clone)]
pub enum Message {
    Selected(usize),
}

#[derive(Default)]
pub struct Model {
    current: usize,
}

impl Model {
    pub fn update(&mut self, Message::Selected(index): Message) {
        self.current = index;
    }

    pub fn view<'a>(&self) -> View<'a, Message> {
        carousel(5, |index| {
            container(text::p((index + 1).to_string())).height(160)
        })
        .width(320)
        .value(self.current)
        .on_value_change(Message::Selected)
        .into()
    }
}
```
