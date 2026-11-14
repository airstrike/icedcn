`.visible(3)` gives each slide one third of the track. The Sizes preview uses two slides below a 1024px window width and three at or above it.

```rust
use icedcn::Slot;
use icedcn::widget::{carousel, text};

pub fn thirds<Message: Clone + 'static>() -> Slot<'static, Message> {
    carousel(5, |index| text::p((index + 1).to_string())).visible(3).width(384).into()
}

pub fn responsive<Message: Clone + 'static>(window_width: f32) -> Slot<'static, Message> {
    let visible = if window_width >= 1024.0 { 3 } else if window_width >= 768.0 { 2 } else { 1 };
    carousel(5, |index| text::p((index + 1).to_string())).visible(visible).width(384).into()
}
```
