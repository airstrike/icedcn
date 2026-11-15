```rust
column![
    input_group(input("Send a message...").disabled(true))
        .addon(input_group::addon(spinner()).align(Align::InlineEnd)),
    input_group(textarea("Send a message...").disabled(true)).addon(
        input_group::addon(
            row![
                spinner(),
                input_group::text("Validating..."),
                space::horizontal(),
                input_group::button(icon::arrow_up())
                    .square()
                    .style(button::Default)
            ]
            .spacing(8)
            .align_y(iced::Center)
            .width(iced::Fill)
        )
        .align(Align::BlockEnd)
    )
]
.width(iced::Fill)
.max_width(448.0)
.spacing(16)
```
