```rust
use icedxdy::widget::{button, command, dialog};

command([
    command::item("Calendar")
        .group("Suggestions")
        .icon(icon::calendar),
    command::item("Search Emoji")
        .group("Suggestions")
        .icon(icon::smile),
    command::item("Calculator")
        .group("Suggestions")
        .icon(icon::calculator)
        .disabled(true),
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
.width(iced::Fill.max(384.0))
```
