```rust
use iced::widget::image;
use iced::{ContentFit, Element, Fill, Widget};
use icedcn::widget::aspect_ratio;
use icedcn::{Theme, radius};

fn preview(photo: image::Handle) -> Element<'static, (), Theme> {
    let photo = image(photo)
        .width(Fill)
        .height(Fill)
        .content_fit(ContentFit::Cover)
        .border_radius(radius::LG);
    aspect_ratio(photo.boxed(), 16.0 / 9.0)
        .max_width(384)
        .into()
}
```
