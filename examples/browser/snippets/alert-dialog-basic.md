```rust
use iced::Fill;
use icedxdy::widget::{alert_dialog, button, column};

alert_dialog(
    |a| a.trigger(button("Show Dialog").style(button::Outline)),
    |a| {
        column![
            a.header(
                "Are you absolutely sure?",
                "This action cannot be undone. This will permanently delete your account and remove your data from our servers.",
            ),
            a.footer([
                a.cancel(button("Cancel").style(button::Outline)),
                a.action(button("Continue")),
            ]),
        ]
        .width(Fill)
        .into()
    },
)
```
