```rust
use iced::{Element, Fill, Widget};
use icedcn::{card, Size, Theme};
use icedcn::widget::{button, column, text};

fn preview() -> Element<'static, Message, Theme> {
    card(column![
        card::header("Small Card")
            .description("This card uses the small size variant."),
        card::content(text::small(
            "Use Size::Sm for a compact card with smaller gaps and padding."
        ).line_height(iced::Pixels(20.0))),
        card::footer(
            button("Action").style(button::Outline).size(Size::Sm)
                .on_press(Message::Action).width(Fill),
        ),
    ].width(Fill))
    .size(Size::Sm)
    .width(Fill.max(384))
    .into()
}
```
