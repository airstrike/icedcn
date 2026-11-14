```rust
use icedcn::widget::{button, dropdown_menu, menu, menu_group};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    move |_| menu_group![
        menu::label("My Account"),
        menu::item("Profile").shortcut("⇧⌘P"),
        menu::item("Billing").shortcut("⌘B"),
        menu::item("Settings").shortcut("⌘S"),
        menu::separator(), menu::item("Log out").shortcut("⇧⌘Q"),
    ],
)
```
