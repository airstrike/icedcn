```rust
use iced::{Element, Fill};
use icedcn::{card, Size, Theme};
use icedcn::widget::{button, column, text};

fn small<Message: Clone + 'static>() -> Element<'static, Message, Theme> {
    card(column![
        card::header("Small Card")
            .description("This card uses the small size variant."),
        card::content(text::small(
            "The card component supports a size that can be set to Size::Sm for a more compact appearance."
        ).line_height(iced::Pixels(20.0))),
        card::footer(
            button("Action").style(button::Outline).size(Size::Sm)
                .width(Fill),
        ),
    ].width(Fill))
    .size(Size::Sm)
    .width(Fill.max(384))
    .into()
}
```
