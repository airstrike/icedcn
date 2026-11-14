```rust
use icedcn::widget::{menu, menubar};

menubar([
    menubar::menu("View", {
        let always_show_bookmarks_bar = state.checked.contains(&"Always Show Bookmarks Bar");
        let always_show_full_urls = state.checked.contains(&"Always Show Full URLs");
        move || {
            menu::group([
                menu::checkbox_item(
                    "Always Show Bookmarks Bar",
                    always_show_bookmarks_bar,
                    |v| Choice::Check("Always Show Bookmarks Bar", v),
                )
                .into(),
                menu::checkbox_item("Always Show Full URLs", always_show_full_urls, |v| {
                    Choice::Check("Always Show Full URLs", v)
                })
                .into(),
                menu::separator(),
                menu::item("Reload").shortcut("⌘R").inset(true).into(),
                menu::item("Force Reload")
                    .disabled(true)
                    .shortcut("⇧⌘R")
                    .inset(true)
                    .into(),
            ])
        }
    })
    .width(256),
    menubar::menu("Format", {
        let strikethrough = state.checked.contains(&"Strikethrough");
        let code = state.checked.contains(&"Code");
        let superscript = state.checked.contains(&"Superscript");
        move || {
            menu::group([
                menu::checkbox_item("Strikethrough", strikethrough, |v| {
                    Choice::Check("Strikethrough", v)
                })
                .into(),
                menu::checkbox_item("Code", code, |v| Choice::Check("Code", v)).into(),
                menu::checkbox_item("Superscript", superscript, |v| {
                    Choice::Check("Superscript", v)
                })
                .into(),
            ])
        }
    }),
])
.width(288)
```
