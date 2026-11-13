```rust
use icedxdy::widget::{context_menu, menu};

context_menu(
    |_| area(),
    |_| {
        menu::group([
            menu::group([
                menu::item(with_icon(icon::pencil(), "Edit")).into(),
                menu::item(with_icon(icon::share(), "Share")).into(),
            ]),
            menu::separator(),
            menu::group([menu::item(with_icon(icon::trash(), "Delete"))
                .style(menu::Destructive)
                .into()]),
        ])
    },
)
```
