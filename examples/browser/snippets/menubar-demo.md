```rust
use icedcn::widget::{menu, menubar};

menubar([
    menubar::menu("File", || {
        menu::group([
            menu::group([
                menu::item("New Tab").shortcut("⌘T").into(),
                menu::item("New Window").shortcut("⌘N").into(),
                menu::item("New Incognito Window").disabled(true).into(),
            ]),
            menu::separator(),
            menu::group([menu::sub(
                "Share",
                [menu::group([
                    menu::item("Email link").into(),
                    menu::item("Messages").into(),
                    menu::item("Notes").into(),
                ])],
            )]),
            menu::separator(),
            menu::group([menu::item("Print...").shortcut("⌘P").into()]),
        ])
    }),
    menubar::menu("Edit", || {
        menu::group([
            menu::group([
                menu::item("Undo").shortcut("⌘Z").into(),
                menu::item("Redo").shortcut("⇧⌘Z").into(),
            ]),
            menu::separator(),
            menu::group([menu::sub(
                "Find",
                [
                    menu::group([menu::item("Search the web").into()]),
                    menu::separator(),
                    menu::group([
                        menu::item("Find...").into(),
                        menu::item("Find Next").into(),
                        menu::item("Find Previous").into(),
                    ]),
                ],
            )]),
            menu::separator(),
            menu::group([
                menu::item("Cut").into(),
                menu::item("Copy").into(),
                menu::item("Paste").into(),
            ]),
        ])
    }),
    menubar::menu("View", {
        let bookmarks_bar = state.checked.contains(&"Bookmarks Bar");
        let full_urls = state.checked.contains(&"Full URLs");
        move || {
            menu::group([
                menu::group([
                    menu::checkbox_item("Bookmarks Bar", bookmarks_bar, |v| {
                        Choice::Check("Bookmarks Bar", v)
                    })
                    .into(),
                    menu::checkbox_item("Full URLs", full_urls, |v| {
                        Choice::Check("Full URLs", v)
                    })
                    .into(),
                ]),
                menu::separator(),
                menu::group([
                    menu::item("Reload").shortcut("⌘R").inset(true).into(),
                    menu::item("Force Reload")
                        .disabled(true)
                        .shortcut("⇧⌘R")
                        .inset(true)
                        .into(),
                ]),
                menu::separator(),
                menu::group([menu::item("Toggle Fullscreen").inset(true).into()]),
                menu::separator(),
                menu::group([menu::item("Hide Sidebar").inset(true).into()]),
            ])
        }
    })
    .width(176),
    menubar::menu("Profiles", {
        let user = state.user;
        move || {
            menu::group([
                menu::group([
                    menu::radio_item("Andy", "andy", Some(user), Choice::User).into(),
                    menu::radio_item("Benoit", "benoit", Some(user), Choice::User).into(),
                    menu::radio_item("Luis", "Luis", Some(user), Choice::User).into(),
                ]),
                menu::separator(),
                menu::group([menu::item("Edit...").inset(true).into()]),
                menu::separator(),
                menu::group([menu::item("Add Profile...").inset(true).into()]),
            ])
        }
    }),
])
.width(288)
```
