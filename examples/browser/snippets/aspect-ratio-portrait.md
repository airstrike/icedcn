```rust
use iced::widget::image;
use iced::{ContentFit, Fill, Widget};
use icedcn::radius;
use icedcn::widget::aspect_ratio;

fn preview<'a>(photo: image::Handle) -> aspect_ratio::AspectRatio<'a, ()> {
    let photo = image(photo)
        .width(Fill)
        .height(Fill)
        .content_fit(ContentFit::Cover)
        .border_radius(radius::LG);
    aspect_ratio(photo.boxed(), 9.0 / 16.0).max_width(160)
}
```
