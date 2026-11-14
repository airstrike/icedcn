## Usage

```rust
use iced::widget::image;
use icedcn::Theme;
use icedcn::widget::avatar;

fn preview(image: image::Handle) -> iced::Element<'static, (), Theme> {
    avatar(Some(image)).fallback("AC").into()
}
```
