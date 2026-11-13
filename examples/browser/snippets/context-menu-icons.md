```rust
use icedxdy::widget::{context_menu, menu};

context_menu(
    |_| area(),
    |_| {
        menu::group([
            menu::group([
                menu::item(with_icon(icon::copy(), "Copy")).into(),
                menu::item(with_icon(icon::scissors(), "Cut")).into(),
                menu::item(with_icon(icon::clipboard_paste(), "Paste")).into(),
            ]),
            menu::separator(),
            menu::group([menu::item(with_icon(icon::trash(), "Delete"))
                .style(menu::Destructive)
                .into()]),
        ])
    },
)
```
