```rust
use icedcn::widget::{button, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    move |_| {
        menu::group![
            menu::label("My Account"),
            menu::item("Profile").shortcut("⇧⌘P"),
            menu::item("Billing").shortcut("⌘B"),
            menu::item("Settings").shortcut("⌘S"),
            menu::separator(),
            menu::item("Team"),
            menu::sub![
                "Invite users",
                menu::item("Email"),
                menu::item("Message"),
                menu::separator(),
                menu::item("More...")
            ],
            menu::item("New Team").shortcut("⌘+T"),
            menu::separator(),
            menu::item("GitHub"),
            menu::item("Support"),
            menu::item("API").disabled(true),
            menu::separator(),
            menu::item("Log out").shortcut("⇧⌘Q"),
        ]
    },
)
.align(dropdown_menu::Align::Start)
.width(160)
```
