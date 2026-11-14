```rust
use iced::widget::image;
use icedcn::Theme;
use icedcn::widget::avatar;

fn preview(portrait: image::Handle) -> iced::Element<'static, (), Theme> {
    avatar(Some(portrait)).fallback("AC").into()
}
```
