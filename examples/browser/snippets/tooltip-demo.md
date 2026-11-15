```rust
use icedcn::{
    Slot,
    widget::{button, tooltip},
};

pub fn view<'a>() -> Slot<'a, ()> {
    tooltip(button("Hover").style(button::Outline), "Add to library").into()
}
```
