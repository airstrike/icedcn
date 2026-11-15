```rust
use icedcn::widget::{button, dropdown_menu, menu, menu_group, menu_sub};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    move |_| {
        menu_group![
            menu::item("Team"),
            menu_sub![
                "Invite users",
                menu::item("Email"),
                menu::item("Message"),
                menu_sub![
                    "More options",
                    menu::item("Calendly"),
                    menu::item("Slack"),
                    menu::separator(),
                    menu::item("Webhook")
                ],
                menu::separator(),
                menu::item("Advanced..."),
            ],
            menu::item("New Team").shortcut("⌘+T"),
        ]
    },
)
```
