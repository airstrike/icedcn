```rust
use iced::{Fill, Widget};
use iced::widget::svg;
use icedcn::Theme;
use icedcn::widget::{alert_dialog, button, column};

alert_dialog(
    |a| a.trigger(button("Share Project").style(button::Outline)),
    |a| {
        column![
            a.header(
                "Share this project?",
                "Anyone with the link will be able to view and edit this project.",
            )
            .media(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 2a10 10 0 0 1 7.38 16.75" /> <path d="M12 8v8" /> <path d="M16 12H8" /> <path d="M2.5 8.875a10 10 0 0 0-.5 3" /> <path d="M2.83 16a10 10 0 0 0 2.43 3.4" /> <path d="M4.636 5.235a10 10 0 0 1 .891-.857" /> <path d="M8.644 21.42a10 10 0 0 0 7.631-.38" /> </svg>"#.as_bytes()))
                .width(24)
                .height(24)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
                .boxed()),
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
