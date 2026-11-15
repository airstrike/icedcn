## Usage

```rust
use icedcn::widget::command;

command([
    command::item("Calendar").group("Suggestions"),
    command::item("Search Emoji").group("Suggestions"),
    command::item("Calculator").group("Suggestions"),
    command::item("Profile").group("Settings"),
    command::item("Billing").group("Settings"),
    command::item("Settings").group("Settings"),
])
.width(iced::Fill.max(384.0))
```

Attach `.on_select(Message::OpenCalendar)` to an item and handle `OpenCalendar` in your application's `update` to run that action. The query and highlighted item stay in the command's widget tree. Shortcut labels are visual hints; register application shortcuts with your iced subscription.
