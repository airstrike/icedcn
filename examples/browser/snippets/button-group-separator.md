```rust
use icedcn::Size::Sm;
use icedcn::Slot;
use icedcn::widget::button;
use icedcn::widget::button::group;

pub fn view<'a>() -> Slot<'a, ()> {
    button::group![
        button("Copy").style(button::Secondary).size(Sm),
        group::separator(),
        button("Paste").style(button::Secondary).size(Sm),
    ]
    .into()
}
```
