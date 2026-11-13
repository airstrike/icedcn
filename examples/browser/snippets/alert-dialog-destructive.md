```rust
use icedxdy::theme::tweak::{Alpha, Bg, Fg, Token};
use icedxdy::widget::{alert_dialog, button, column};

alert_dialog(
    |a| a.trigger(button("Delete Chat").style(button::Destructive)),
    |a| {
        column![
            a.header_content(
                "Delete chat?",
                rich_text([
                    span("This will permanently delete this chat conversation. View "),
                    span("Settings").link(()).underline(true),
                    span(" delete any memories saved during this chat."),
                ])
                .size(14)
                .align_x(iced::Center)
                .style(|theme| muted(theme).into())
                .on_link_click(|()| Message::OpenSettings)
                .boxed(),
            )
            .media(icon::trash_2())
            .media_style(alert_dialog::Media + Bg(Token::Destructive) + Alpha(0.1) + Fg(Token::Destructive)),
            a.footer([
                a.cancel(button("Cancel").style(button::Outline)),
                a.action(button("Delete").style(button::Destructive).on_press(Message::Delete)),
            ]),
        ]
        .width(Fill)
        .into()
    },
)
.size(Size::Sm)
```
