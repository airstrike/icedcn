```rust
use crate::icon;
use icedcn::Slot;
use icedcn::widget::{button, button_group, input};

pub fn view(window_width: f32) -> Slot<'static, ()> {
    let narrow = window_width < 768.;
    button_group![
        input("Search...")
            .text_size(if narrow { 16. } else { 14. })
            .width(if narrow { 216. } else { 192. }),
        button(icon::search().size(16).line_height(1.0)).style(button::Outline)
    ]
    .into()
}
```
