## Usage

```rust
use iced::widget::image;
use iced::{ContentFit, Fill, Widget};
use icedcn::widget::aspect_ratio;
use icedcn::radius;

fn photo(handle: image::Handle) -> aspect_ratio::AspectRatio<'static, ()> {
    let photo = image(handle)
        .width(Fill)
        .height(Fill)
        .border_radius(radius::MD)
        .content_fit(ContentFit::Cover);

    aspect_ratio(photo.boxed(), 16.0 / 9.0)
}
```

Pass the image for the active theme into each example. The gallery uses `assets/landscape.jpg` in light mode and `assets/landscape-dark.png` in dark mode. The dark asset contains the same photo at 20% brightness.
