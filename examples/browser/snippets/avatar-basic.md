```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::widget::avatar;

fn preview(portrait: image::Handle) -> iced::Element<'static, (), Theme> {
    avatar(Some(portrait)).fallback("AC").into()
}
```
