```rust
use icedxdy::widget::{button, command, dialog};

dialog(
    |d| d.trigger(button("Open Menu").style(button::Outline)),
    |_| {
        command([
            command::item("Calendar").group("Suggestions"),
            command::item("Search Emoji").group("Suggestions"),
            command::item("Calculator").group("Suggestions"),
        ])
        .into()
    },
)
.show_close_button(false)
.max_width(512.0)
```
