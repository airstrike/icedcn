```rust
use icedcn::widget::{button, command};

command::dialog(
    |d| d.trigger(button("Open Menu").style(button::Outline)),
    || {
        command([
            command::item("Calendar").group("Suggestions"),
            command::item("Search Emoji").group("Suggestions"),
            command::item("Calculator").group("Suggestions"),
        ])
    },
)
```
