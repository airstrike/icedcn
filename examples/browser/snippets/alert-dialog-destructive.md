```rust
use iced::{Fill, Widget};
use iced::widget::svg;
use icedcn::Theme;
use icedcn::Size;
use iced::widget::{rich_text, span, text::LineHeight};
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
            .media(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /> <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" /> <line x1="10" x2="10" y1="11" y2="17" /> <line x1="14" x2="14" y1="11" y2="17" /> </svg>"#.as_bytes()))
                .width(24)
                .height(24)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.destructive.base.into()) })
                .boxed())
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
