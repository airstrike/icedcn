```rust
use icedcn::{
    Slot,
    widget::{button, tooltip},
};

pub fn view() -> Slot<'static, ()> {
    tooltip(button("Hover").style(button::Outline), "Add to library").into()
}
```
