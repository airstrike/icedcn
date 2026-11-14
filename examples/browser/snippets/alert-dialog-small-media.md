```rust
use iced::{Fill, Widget};
use iced::widget::svg;
use icedcn::Theme;
use icedcn::Size;
use icedcn::widget::{alert_dialog, button, column};

alert_dialog(
    |a| a.trigger(button("Show Dialog").style(button::Outline)),
    |a| {
        column![
            a.header(
                "Allow accessory to connect?",
                "Do you want to allow the USB accessory to connect to this device?",
            )
            .media(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m7 7 10 10-5 5V2l5 5L7 17" /> </svg>"#.as_bytes()))
                .width(24)
                .height(24)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
                .boxed()),
            a.footer([
                a.cancel(button("Don't allow").style(button::Outline)),
                a.action(button("Allow")),
            ]),
        ]
        .width(Fill)
        .into()
    },
)
.size(Size::Sm)
```
