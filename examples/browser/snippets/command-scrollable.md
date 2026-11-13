```rust
use icedxdy::widget::{button, command, dialog};

dialog(
    |d| d.trigger(button("Open Menu").style(button::Outline)),
    |_| {
        command([
            command::item("Home")
                .group("Navigation")
                .icon(icon::house)
                .shortcut("⌘H"),
            command::item("Inbox")
                .group("Navigation")
                .icon(icon::inbox)
                .shortcut("⌘I"),
            command::item("Documents")
                .group("Navigation")
                .icon(icon::file_text)
                .shortcut("⌘D"),
            command::item("Folders")
                .group("Navigation")
                .icon(icon::folder)
                .shortcut("⌘F"),
            command::item("New File")
                .group("Actions")
                .icon(icon::plus)
                .shortcut("⌘N"),
            command::item("New Folder")
                .group("Actions")
                .icon(icon::folder_plus)
                .shortcut("⇧⌘N"),
            command::item("Copy")
                .group("Actions")
                .icon(icon::copy)
                .shortcut("⌘C"),
            command::item("Cut")
                .group("Actions")
                .icon(icon::scissors)
                .shortcut("⌘X"),
            command::item("Paste")
                .group("Actions")
                .icon(icon::clipboard_paste)
                .shortcut("⌘V"),
            command::item("Delete")
                .group("Actions")
                .icon(icon::trash)
                .shortcut("⌫"),
            command::item("Grid View")
                .group("View")
                .icon(icon::layout_grid),
            command::item("List View").group("View").icon(icon::list),
            command::item("Zoom In")
                .group("View")
                .icon(icon::zoom_in)
                .shortcut("⌘+"),
            command::item("Zoom Out")
                .group("View")
                .icon(icon::zoom_out)
                .shortcut("⌘-"),
            command::item("Profile")
                .group("Account")
                .icon(icon::user)
                .shortcut("⌘P"),
            command::item("Billing")
                .group("Account")
                .icon(icon::credit_card)
                .shortcut("⌘B"),
            command::item("Settings")
                .group("Account")
                .icon(icon::settings)
                .shortcut("⌘S"),
            command::item("Notifications")
                .group("Account")
                .icon(icon::bell),
            command::item("Help & Support")
                .group("Account")
                .icon(icon::circle_question_mark),
            command::item("Calculator")
                .group("Tools")
                .icon(icon::calculator),
            command::item("Calendar")
                .group("Tools")
                .icon(icon::calendar),
            command::item("Image Editor")
                .group("Tools")
                .icon(icon::image),
            command::item("Code Editor").group("Tools").icon(icon::code),
        ])
        .into()
    },
)
.show_close_button(false)
.max_width(512.0)
```
