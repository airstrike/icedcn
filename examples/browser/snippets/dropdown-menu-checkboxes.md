```rust
use icedcn::View;
use icedcn::widget::{button, dropdown_menu, menu};

pub fn view<'a>(checked: [bool; 3]) -> View<'a, [bool; 3]> {
    let toggle = move |i: usize, value| {
        let mut next = checked;
        next[i] = value;
        next
    };
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu::group![
                menu::label("Appearance"),
                menu::checkbox_item("Status Bar", checked[0], |v| toggle(0, v)),
                menu::checkbox_item("Activity Bar", checked[1], |v| toggle(1, v)).disabled(true),
                menu::checkbox_item("Panel", checked[2], |v| toggle(2, v)),
            ]
        },
    )
    .width(160)
    .into()
}
```

Initialize the application value to `[true, false, false]`. Keep the value this menu publishes when an item is selected.
