## Usage

```rust
use iced::widget::image;
use iced::{ContentFit, Element, Fill, Widget};
use icedxdy::widget::aspect_ratio;
use icedxdy::{Theme, radius};

fn photo(handle: image::Handle) -> Element<'static, (), Theme> {
    let photo = image(handle)
        .width(Fill)
        .height(Fill)
        .border_radius(radius::MD)
        .content_fit(ContentFit::Cover);

    aspect_ratio(photo.boxed(), 16.0 / 9.0).into()
}
```

Pass the image for the active theme into each example. The gallery uses `assets/landscape.jpg` in light mode and `assets/landscape-dark.png` in dark mode. The dark asset contains the same photo at 20% brightness.
