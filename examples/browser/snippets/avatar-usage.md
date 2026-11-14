## Usage

```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::widget::avatar;

fn preview(image: image::Handle) -> iced::Element<'static, (), Theme> {
    avatar(Some(image)).fallback("AC").into()
}
```
