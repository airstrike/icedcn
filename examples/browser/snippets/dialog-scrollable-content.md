```rust
use icedcn::widget::{button, column, dialog, text};

const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

dialog(
    |d| d.trigger(button("Scrollable Content").style(button::Outline)),
    |_| dialog::header("Scrollable Content", "This is a dialog with scrollable content."),
)
.scrollable_body(|_| {
    column((0..10).map(|_| text::small(PARAGRAPH).line_height(1.5).into()))
        .spacing(16)
        .into()
})
```
