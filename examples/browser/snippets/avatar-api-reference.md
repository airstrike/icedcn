### Avatar

The `Avatar` component is the root component that wraps the avatar image and fallback.

| Rust API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `avatar(image)` | `Option<image::Handle>` | required | `Some(handle)` displays an image; `None` displays the fallback. |
| `.size(size)` | `impl Into<avatar::Size>` | `avatar::Default` | `Sm` 24px, default 32px, `Lg` 40px, or a custom diameter. |
| `.style(style)` | `Fn(&Theme) -> container::Style` | muted surface and inset edge | Styles the avatar surface, fallback color and inset border. |

```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::widget::avatar;

fn preview(image: image::Handle) -> iced::Element<'static, (), Theme> {
    avatar(Some(image))
        .fallback("CN")
        .size(avatar::Default)
        .into()
}
```
