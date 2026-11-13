```rust
card(column![
                card::header(card::title("Small Card").size(14.0)).inset(12.0)
                    .description("This card uses the small size variant."),
                card::content(text::p(
                    "The card component supports a size prop that can be set to \"sm\" for a more compact appearance."
                )).padding([0,12]),
                card::footer_with_spacing(
                    button("Action").style(button::Outline)
                        .size(Size::Sm)
                        .on_press(Message::Page(pi, page::Message::Noop))
                        .width(Fill),12.0
                ),
            ]
            .spacing(12)
            .padding(iced::Padding::ZERO.top(12))
            .width(Fill))
            .width(384.0)
            .boxed()
```
