```rust
use crate::icon;
use icedcn::widget::input_group::Align;
use icedcn::widget::{button, column, empty, input, input_group, kbd, row};

empty::empty()
    .title("404 - Not Found")
    .description(
        "The page you're looking for doesn't exist. Try searching for what you need below.",
    )
    .content(
        column![
            input_group(input("Try searching for pages..."))
                .addon(input_group::addon(icon::search()))
                .addon(input_group::addon(kbd("/")).align(Align::InlineEnd)),
            row![
                empty::description("Need help?"),
                button("Contact support")
                    .style(button::Link)
                    .on_press(Message::ContactSupport)
            ]
            .align_y(iced::Center)
        ]
        .spacing(16)
        .align_x(iced::Center)
        .max_width(288.0),
    )
```
