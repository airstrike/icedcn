## Usage

```rust
use iced::widget::image;
use icedcn::widget::avatar;

fn preview(image: image::Handle) -> avatar::Avatar<'static, ()> {
    avatar(Some(image)).fallback("AC")
}
```
