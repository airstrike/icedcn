```rust
use icedcn::Size::Sm;
use icedcn::Slot;
use icedcn::widget::button::group;
use icedcn::widget::{button, button_group};

pub fn view<'a>() -> Slot<'a, ()> {
    button_group![
        button("Copy").style(button::Secondary).size(Sm),
        group::separator(),
        button("Paste").style(button::Secondary).size(Sm),
    ]
    .into()
}
```
