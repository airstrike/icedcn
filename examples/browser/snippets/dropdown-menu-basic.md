```rust
use icedcn::widget::{button, dropdown_menu, menu, menu_group};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    move |_| {
        menu_group![
            menu::label("My Account"),
            menu::item("Profile"),
            menu::item("Billing"),
            menu::item("Settings"),
            menu::separator(),
            menu::item("GitHub"),
            menu::item("Support"),
            menu::item("API").disabled(true),
        ]
    },
)
```
