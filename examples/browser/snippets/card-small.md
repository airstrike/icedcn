```rust
use iced::Fill;
use icedcn::widget::{button, column, text};
use icedcn::{Size, card};

fn small<'a, Message: Clone + 'a>() -> card::Card<'a, Message> {
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
}
```
