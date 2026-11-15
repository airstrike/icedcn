```rust
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
                    .on_press(Message::Page(
                        self.page_index(),
                        page::Message::LinkClicked("#".into())
                    ))
            ]
            .align_y(iced::Center)
        ]
        .spacing(16)
        .align_x(iced::Center)
        .max_width(288.0),
    )
```
