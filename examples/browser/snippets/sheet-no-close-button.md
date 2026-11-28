```rust
use icedcn::{
    View,
    widget::{button, sheet},
};
pub fn view<'a>() -> View<'a> {
    sheet(
                |s| s.trigger(button("Open Sheet").style(button::Outline)),
                |_| sheet::header(
                    "No Close Button",
                    "This sheet doesn't have a close button in the top-right corner. Click outside to close.",
                ),
            ).show_close_button(false).into()
}
```
