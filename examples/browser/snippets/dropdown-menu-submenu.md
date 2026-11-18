```rust
use icedcn::widget::{button, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    move |_| {
        menu::group![
            menu::item("Team"),
            menu::sub![
                "Invite users",
                menu::item("Email"),
                menu::item("Message"),
                menu::sub![
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
