```rust
use iced::widget::{container, image, space, stack};
use iced::{Color, ContentFit, Element, Fill, Widget};

use icedxdy::card;
use icedxdy::widget::{aspect_ratio, badge, button, column};
use icedxdy::{Gap, Theme};

fn preview(photo: image::Handle) -> Element<'static, Message, Theme> {
    let photo = image(photo)
        .width(Fill)
        .height(Fill)
        .content_fit(ContentFit::Cover);
    let tint = container(space())
        .width(Fill)
        .height(Fill)
        .style(|theme: &Theme| {
            let opacity = if theme.neutral.is_dark { 0.74 } else { 0.61 };
            container::Style::default().background(Color::BLACK.scale_alpha(opacity))
        });

    card(column![
        aspect_ratio(stack![photo, tint].boxed(), 16.0 / 9.0),
        card::header("Design systems meetup")
            .description(
                "A practical talk on component APIs, accessibility, and shipping faster.",
            )
            .action(card::action(badge("Featured").style(badge::Secondary))),
        card::footer(
            button("View Event")
                .on_press(Message::View)
                .width(Fill)
        ),
    ]
    .spacing(Gap::G4)
    .width(Fill))
    .width(384.0)
    .clip(true)
    .boxed()
}
```
