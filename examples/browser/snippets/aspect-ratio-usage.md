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

Pass a grayscale image handle to the examples below, such as `image::Handle::from_path("assets/shadcn1.jpg")` when running from the browser example directory.
