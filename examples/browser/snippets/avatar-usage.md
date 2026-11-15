## Usage

```rust
use iced::widget::image;
use icedcn::widget::avatar;

fn view<'a>(image: image::Handle) -> avatar::Avatar<'a, ()> {
    avatar(image).fallback("AT")
}
```
