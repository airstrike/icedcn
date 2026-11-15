```rust
use iced::widget::image;
use icedcn::widget::avatar;

fn view<'a>(portrait: image::Handle) -> avatar::Avatar<'a, ()> {
    avatar(portrait).fallback("AT")
}
```
