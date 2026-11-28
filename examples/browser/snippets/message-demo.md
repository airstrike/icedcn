```rust
use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use icedcn::widget::bubble::Muted;
use icedcn::widget::status_marker::shimmer;
use icedcn::widget::{bubble, column, message, status_marker};

frame(
    column![
        message(bubble("Deploying to prod real quick."))
            .align(Right)
            .avatar(portrait(2, "ME")),
        message(bubble("It's 4:55 PM. On a Friday.").style(Muted)).avatar(portrait(0, "R")),
        message(bubble("It's a one-line change."))
            .align(Right)
            .avatar(portrait(2, "ME"))
            .footer("Delivered"),
        message(bubble::group([
            bubble("It's always a one-line change 😭.")
                .style(Muted)
                .into(),
            bubble("Alright, let me take a look.")
                .style(Muted)
                .reactions("👍", Right, Bottom)
                .into()
        ]))
        .avatar(portrait(0, "R")),
        status_marker(shimmer("Oliver is typing..."))
    ]
    .spacing(24),
)
```
