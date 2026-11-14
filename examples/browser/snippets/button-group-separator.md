```rust
use icedcn::Slot;
use icedcn::Size::Sm;
use icedcn::widget::{button, button_group};
use icedcn::widget::button::group;

pub fn view() -> Slot<'static, ()> {
    button_group![
        button("Copy").style(button::Secondary).size(Sm),
        group::separator(),
        button("Paste").style(button::Secondary).size(Sm),
    ]
    .into()
}
```
