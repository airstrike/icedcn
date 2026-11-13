```rust
use icedxdy::widget::{context_menu, menu};

context_menu(
    |_| area(),
    |_| {
        menu::group([
            menu::group([
                menu::item("Back").shortcut("⌘[").into(),
                menu::item("Forward").disabled(true).shortcut("⌘]").into(),
                menu::item("Reload").shortcut("⌘R").into(),
            ]),
            menu::separator(),
            menu::group([
                menu::item("Save").shortcut("⌘S").into(),
                menu::item("Save As...").shortcut("⇧⌘S").into(),
            ]),
        ])
    },
)
```
