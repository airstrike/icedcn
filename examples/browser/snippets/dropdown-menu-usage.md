## Usage

The trigger opens the menu. Selecting an enabled item closes it and publishes any message set with `.on_select(...)`.

```rust
use icedcn::widget::{button, dropdown_menu, menu, menu_group};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    move |_| menu_group![
        menu::label("My Account"), menu::item("Profile"), menu::item("Billing"),
        menu::separator(), menu::item("Team"), menu::item("Subscription"),
    ],
)
```
