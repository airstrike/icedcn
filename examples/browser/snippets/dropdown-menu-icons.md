```rust
use icedcn::widget::{button, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    |_| {
        menu::group([
            menu::item(with_icon(icon::user(), "Profile")).into(),
            menu::item(with_icon(icon::credit_card(), "Billing")).into(),
            menu::item(with_icon(icon::settings(), "Settings")).into(),
            menu::separator(),
            menu::item(with_icon(icon::log_out(), "Log out"))
                .style(menu::Destructive)
                .into(),
        ])
    },
)
```
