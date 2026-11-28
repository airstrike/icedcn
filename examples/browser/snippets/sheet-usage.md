## Usage

Pass a trigger and a content closure to `sheet`. The sheet opens at the right edge by default.

```rust
use icedcn::{
    View,
    widget::{button, sheet},
};

pub fn view<'a>() -> View<'a> {
    sheet(
        |s| s.trigger(button("Open").style(button::Outline)),
        |_| sheet::header("Are you absolutely sure?", "This action cannot be undone."),
    )
    .into()
}
```
