Use `.badge_style(badge::presence)` for a green status dot, or pass a style function to customize its surface and text color. Badge geometry follows `.size(...)` on its Avatar.

```rust
use icedcn::widget::avatar::badge;
use iced::widget::image;
use icedcn::widget::avatar;

fn view<'a>(hecrj: image::Handle) -> avatar::Avatar<'a, ()> {
    avatar::badge(avatar(hecrj).fallback("HR"), avatar::dot())
        .badge_style(badge::presence)
}
```
