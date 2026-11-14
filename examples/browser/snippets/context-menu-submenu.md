```rust
use icedcn::widget::{context_menu, menu};

context_menu(
    |_| area(),
    |_| {
        menu::group([
            menu::group([
                menu::item("Copy").shortcut("⌘C").into(),
                menu::item("Cut").shortcut("⌘X").into(),
            ]),
            menu::sub(
                "More Tools",
                [
                    menu::group([
                        menu::item("Save Page...").into(),
                        menu::item("Create Shortcut...").into(),
                        menu::item("Name Window...").into(),
                    ]),
                    menu::separator(),
                    menu::group([menu::item("Developer Tools").into()]),
                    menu::separator(),
                    menu::group([menu::item("Delete").style(menu::Destructive).into()]),
                ],
            ),
        ])
    },
)
```
