```rust
use icedcn::View;
use icedcn::theme::{Bg, Destructive, Fg, hover, radius};
use icedcn::widget::{button, row};

pub fn tweaked<'a>() -> View<'a> {
    let quiet = button::Ghost + Fg::MutedForeground + hover(Fg::Foreground);
    row![
        button("Muted").style(Destructive + Bg::Muted),
        button("Square").style(button::Outline + radius(0)),
        button("Quiet").style(quiet),
    ]
    .spacing(8)
    .into()
}
```
