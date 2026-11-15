Select the light or dark cover when your application theme changes. Prepare the grayscale image with brightness 60% (light) or 40% (dark), then a 35% black shade, before drawing its rounded corners.

```rust
use iced::border;
use iced::widget::image;
use iced::{ContentFit, Fill, Widget};
use icedcn::widget::{aspect_ratio, badge, button, column};
use icedcn::{Theme, card, radius};

fn event_card<Message: Clone + 'static>(
    light: image::Handle,
    dark: image::Handle,
    theme: &Theme,
) -> card::Card<'static, Message> {
    let photo = if theme.neutral.is_dark { dark } else { light };
    let photo = image(photo)
        .width(Fill)
        .height(Fill)
        .content_fit(ContentFit::Cover)
        .border_radius(border::Radius {
            top_left: radius::XL,
            top_right: radius::XL,
            ..Default::default()
        });

    card(
        column![
            aspect_ratio(photo.boxed(), 16.0 / 9.0),
            card::header("Design systems meetup")
                .description(
                    "A practical talk on component APIs, accessibility, and shipping faster."
                )
                .action(badge("Featured").style(badge::Secondary)),
            card::footer(button("View Event").width(Fill)),
        ]
        .width(Fill),
    )
    .padding(0)
    .width(Fill.max(384))
}
```
