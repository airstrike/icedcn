## State and messages

Let a control own state that only affects itself. For example, `checkbox().default_checked(true)` starts checked and keeps its value in the widget tree. A form that validates or submits several values should own those values together.

This controlled checkbox displays the application's value and publishes the complete next boolean:

```rust
use icedcn::View;
use icedcn::widget::{checkbox, field};

#[derive(Default)]
pub struct Preferences {
    notifications: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Notifications(bool),
}

impl Preferences {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Notifications(value) => self.notifications = value,
        }
    }

    pub fn view(&self) -> View<'_, Message> {
        field(
            checkbox()
                .checked(self.notifications)
                .on_checked_change(Message::Notifications),
        )
        .label("Enable notifications")
        .into()
    }
}
```

Use the same pattern for `.value(...).on_input(...)` and `.open(...).on_open_change(...)`. Updating a default after a control has mounted does not replace its current value; use its controlled builder when the displayed state must follow the application.
