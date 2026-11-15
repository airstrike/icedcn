## Usage

```rust
use iced::widget::image;
use icedcn::widget::avatar;

fn view<'a>(image: image::Handle) -> avatar::Avatar<'a, ()> {
    avatar(image).fallback("AT")
}
```

Icon examples import `crate::icon`; follow the shared [icon setup](icedcn://guide/installation#icons) to generate that module and register its font.
