```rust
use crate::icon;
use icedcn::Slot;
use icedcn::widget::button;
use icedcn::widget::button::group;

pub fn view<'a>() -> Slot<'a, ()> {
    button::group![
        button(icon::plus().size(16).line_height(1.0))
            .square()
            .style(button::Outline),
        button(icon::minus().size(16).line_height(1.0))
            .square()
            .style(button::Outline),
    ]
    .orientation(group::Vertical)
    .into()
}
```
