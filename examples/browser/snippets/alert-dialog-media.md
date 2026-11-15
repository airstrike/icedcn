```rust
use crate::icon;
use iced::Fill;
use icedcn::widget::{alert_dialog, button, column};

alert_dialog(
    |a| a.trigger(button("Share Project").style(button::Outline)),
    |a| {
        column![
            a.header(
                "Share this project?",
                "Anyone with the link will be able to view and edit this project.",
            )
            .media(icon::circle_fading_plus().size(24).line_height(1.0)),
            a.footer([
                a.cancel(button("Cancel").style(button::Outline)),
                a.action(button("Share")),
            ]),
        ]
        .width(Fill)
        .into()
    },
)
```
