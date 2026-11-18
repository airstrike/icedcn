`.spacing(16)` leaves 16 pixels between adjacent slides. Choose a gutter from the available width to vary it when the window resizes.

```rust
use icedcn::Slot;
use icedcn::widget::{carousel, text};

pub fn fixed<'a, Message: Clone + 'static>() -> Slot<'a, Message> {
    carousel(5, |index| text::p((index + 1).to_string()))
        .visible(3)
        .spacing(16.0)
        .width(384)
        .into()
}

pub fn responsive<'a, Message: Clone + 'static>(window_width: f32) -> Slot<'a, Message> {
    carousel(5, |index| text::p((index + 1).to_string()))
        .visible(3)
        .spacing(if window_width >= 768.0 { 16.0 } else { 8.0 })
        .width(384)
        .into()
}
```
