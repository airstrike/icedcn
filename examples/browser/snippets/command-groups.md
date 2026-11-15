```rust
use crate::icon;
use icedcn::widget::{button, command};

command::dialog(
    |d| d.trigger(button("Open Menu").style(button::Outline)),
    || {
        command([
            command::item("Calendar")
                .group("Suggestions")
                .icon(icon::calendar),
            command::item("Search Emoji")
                .group("Suggestions")
                .icon(icon::smile),
            command::item("Calculator")
                .group("Suggestions")
                .icon(icon::calculator),
            command::item("Profile")
                .group("Settings")
                .icon(icon::user)
                .shortcut("⌘P"),
            command::item("Billing")
                .group("Settings")
                .icon(icon::credit_card)
                .shortcut("⌘B"),
            command::item("Settings")
                .group("Settings")
                .icon(icon::settings)
                .shortcut("⌘S"),
        ])
    },
)
```
