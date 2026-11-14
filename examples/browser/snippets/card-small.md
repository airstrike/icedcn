```rust
use iced::{Element, Fill, Widget};
use icedxdy::{card, Size, Theme};
use icedxdy::widget::{button, column, text};

fn preview() -> Element<'static, Message, Theme> {
    card(column![
        card::header(card::title("Small Card").size(14.0))
            .inset(12.0)
            .description("This card uses the small size variant."),
        card::content(text::small(
            "The card component supports a size prop that can be set to \"sm\" for a more compact appearance."
        ).line_height(iced::Pixels(20.0))).padding([0, 12]),
        card::footer_with_spacing(
            button("Action").style(button::Outline).size(Size::Sm)
                .on_press(Message::Action).width(Fill),
            12.0,
        ),
    ].spacing(12).padding(iced::Padding::ZERO.top(12)).width(Fill))
    .width(Fill.max(384))
    .boxed()
}
```
