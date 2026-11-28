```rust
use icedcn::View;
use icedcn::widget::{button, row, tooltip};
use tooltip::Side::{Bottom, Left, Right, Top};

pub fn view<'a>() -> View<'a> {
    let hint =
        |label, side| tooltip(button(label).style(button::Outline), "Add to library").side(side);
    row![
        hint("Left", Left),
        hint("Top", Top),
        hint("Bottom", Bottom),
        hint("Right", Right)
    ]
    .spacing(8)
    .wrap()
    .into()
}
```
