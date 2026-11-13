```rust
use icedxdy::widget::{context_menu, menu};

context_menu(
    |_| area(),
    |_| {
        menu::group([
            menu::group([
                menu::label("File"),
                menu::item("New File").shortcut("⌘N").into(),
                menu::item("Open File").shortcut("⌘O").into(),
                menu::item("Save").shortcut("⌘S").into(),
            ]),
            menu::separator(),
            menu::group([
                menu::label("Edit"),
                menu::item("Undo").shortcut("⌘Z").into(),
                menu::item("Redo").shortcut("⇧⌘Z").into(),
            ]),
            menu::separator(),
            menu::group([
                menu::item("Cut").shortcut("⌘X").into(),
                menu::item("Copy").shortcut("⌘C").into(),
                menu::item("Paste").shortcut("⌘V").into(),
            ]),
            menu::separator(),
            menu::group([menu::item("Delete")
                .style(menu::Destructive)
                .shortcut("⌫")
                .into()]),
        ])
    },
)
```
