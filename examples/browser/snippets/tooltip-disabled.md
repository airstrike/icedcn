```rust
use icedcn::Slot;
use icedcn::widget::{button, tooltip};

pub fn view<'a>() -> Slot<'a, ()> {
    tooltip(
        button("Disabled").style(button::Outline).disabled(true),
        "This feature is currently unavailable",
    )
    .into()
}
```
