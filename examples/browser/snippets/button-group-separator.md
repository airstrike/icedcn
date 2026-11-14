```rust
use icedcn::Size::Sm;
use icedcn::widget::button;

button_group![
    button("Copy").style(button::Secondary).size(Sm).on_press(Message::Copy),
    button::group::separator(),
    button("Paste").style(button::Secondary).size(Sm).on_press(Message::Paste),
]
```
