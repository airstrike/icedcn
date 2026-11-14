```rust
use icedcn::widget::{menu, menubar};

menubar([
    menubar::menu("File", || {
        menu::group([
            menu::item(with_icon(icon::file(), "New File"))
                .shortcut("⌘N")
                .into(),
            menu::item(with_icon(icon::folder(), "Open Folder")).into(),
            menu::separator(),
            menu::item(with_icon(icon::save(), "Save"))
                .shortcut("⌘S")
                .into(),
        ])
    }),
    menubar::menu("More", || {
        menu::group([menu::group([
            menu::item(with_icon(icon::settings(), "Settings")).into(),
            menu::item(with_icon(icon::circle_question_mark(), "Help")).into(),
            menu::separator(),
            menu::item(with_icon(icon::trash(), "Delete"))
                .style(menu::Destructive)
                .into(),
        ])])
    }),
])
.width(288)
```
