```rust
use icedcn::widget::{button, dropdown_menu, menu, menu_group};
use icedcn::Slot;

pub fn view(selected: &'static str) -> Slot<'static, &'static str> {
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| menu_group![
            menu::label("Panel Position"),
            menu::radio_item("Top", "top", Some(selected), |v| v),
            menu::radio_item("Bottom", "bottom", Some(selected), |v| v),
            menu::radio_item("Right", "right", Some(selected), |v| v),
        ],
    ).width(128).into()
}
```

Initialize the application value to `"bottom"`. Keep the value this menu publishes when an item is selected.
