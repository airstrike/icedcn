```rust
use crate::icon;
use icedcn::Slot;
use icedcn::widget::{button, button_group};
use icedcn::widget::button::group;

pub fn view() -> Slot<'static, ()> {
    button_group![
        button("Button").style(button::Secondary),
        group::separator(),
        button(icon::plus().size(16).line_height(1.0)).square().style(button::Secondary)
    ]
    .into()
}
```
