```rust
use icedcn::View;
use icedcn::widget::{button, tooltip};

pub fn view<'a>() -> View<'a> {
    tooltip(
        button("Disabled").style(button::Outline).disabled(true),
        "This feature is currently unavailable",
    )
    .into()
}
```
