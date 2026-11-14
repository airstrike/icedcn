```rust
use iced::widget::{container, image, stack};
use iced::{Border, Color, ContentFit, Element, Fill, Widget};
use icedxdy::widget::aspect_ratio;
use icedxdy::{Theme, radius};

fn preview(photo: image::Handle) -> Element<'static, (), Theme> {
    let photo = image(photo)
        .width(Fill)
        .height(Fill)
        .content_fit(ContentFit::Cover)
        .border_radius(radius::LG);
    let shade = container(iced::widget::space())
        .width(Fill)
        .height(Fill)
        .style(|theme: &Theme| {
            let opacity = if theme.neutral.is_dark { 0.8 } else { 0.0 };
            container::Style::default()
                .background(Color::BLACK.scale_alpha(opacity))
                .border(Border::default().rounded(radius::LG))
        });

    aspect_ratio(stack![photo, shade].boxed(), 9.0 / 16.0)
        .max_width(160)
        .into()
}
```
