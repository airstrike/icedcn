```rust
use iced::widget::image;
use icedcn::widget::avatar;
use icedcn::widget::avatar::badge;

fn view<'a>(hecrj: image::Handle) -> avatar::Avatar<'a, ()> {
    avatar::badge(avatar(hecrj).fallback("HR"), avatar::dot()).badge_style(badge::presence)
}
```
