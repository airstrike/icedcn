```rust
use icedcn::widget::{menu, menubar};

menubar([
    menubar::menu("File", || {
        menu::group([
            menu::sub(
                "Share",
                [
                    menu::item("Email link").into(),
                    menu::item("Messages").into(),
                    menu::item("Notes").into(),
                ],
            ),
            menu::separator(),
            menu::item("Print...").shortcut("⌘P").into(),
        ])
    }),
    menubar::menu("Edit", || {
        menu::group([
            menu::item("Undo").shortcut("⌘Z").into(),
            menu::item("Redo").shortcut("⇧⌘Z").into(),
            menu::separator(),
            menu::sub(
                "Find",
                [
                    menu::item("Find...").into(),
                    menu::item("Find Next").into(),
                    menu::item("Find Previous").into(),
                ],
            ),
            menu::separator(),
            menu::item("Cut").into(),
            menu::item("Copy").into(),
            menu::item("Paste").into(),
        ])
    }),
])
.width(288)
```
