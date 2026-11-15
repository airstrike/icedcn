```rust
use iced::widget::image;
use icedcn::widget::avatar;

fn preview(portrait: image::Handle) -> avatar::Avatar<'static, ()> {
    avatar(Some(portrait)).fallback("AC")
}
```
