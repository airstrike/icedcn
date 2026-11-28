```rust
use icedcn::{
    View,
    widget::{button, tooltip},
};

pub fn view<'a>() -> View<'a> {
    tooltip(button("Hover").style(button::Outline), "Add to library").into()
}
```
