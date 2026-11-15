```rust
use crate::icon;
use iced::widget::{rich_text, span, text::LineHeight};
use iced::{Fill, Widget};
use icedcn::Size;
use icedcn::theme::text::destructive;
use icedcn::theme::text::muted;
use icedcn::widget::{alert_dialog, button, column};

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
                .selectable(true)
                .size(14)
                .line_height(LineHeight::Absolute(20.0.into()))
                .align_x(iced::Center)
                .style(|theme| muted(theme).into())
                .on_link_click(|()| Message::ScrollToTop)
                .boxed(),
            )
            .media(
                icon::trash_2()
                    .size(24)
                    .line_height(1.0)
                    .style(|theme| destructive(theme).into())
            )
            .media_style(alert_dialog::Destructive),
            a.footer([
                a.cancel(button("Cancel").style(button::Outline)),
                a.action(button("Delete").style(button::Destructive)),
            ]),
        ]
        .width(Fill)
        .into()
    },
)
.size(Size::Sm)
```
