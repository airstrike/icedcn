```rust
use crate::icon;
use icedcn::View;
use icedcn::widget::button;
use icedcn::widget::button::group;

pub fn view<'a>() -> View<'a> {
    button::group![
        button("Button").style(button::Secondary),
        group::separator(),
        button(icon::plus().size(16).line_height(1.0))
            .square()
            .style(button::Secondary)
    ]
    .into()
}
```
