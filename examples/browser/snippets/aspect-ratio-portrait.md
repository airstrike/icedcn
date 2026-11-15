```rust
use iced::widget::image;
use iced::{ContentFit, Fill, Widget};
use icedcn::widget::aspect_ratio;
use icedcn::radius;

fn preview(photo: image::Handle) -> aspect_ratio::AspectRatio<'static, ()> {
    let photo = image(photo)
        .width(Fill)
        .height(Fill)
        .content_fit(ContentFit::Cover)
        .border_radius(radius::LG);
    aspect_ratio(photo.boxed(), 9.0 / 16.0)
        .max_width(160)
}
```
