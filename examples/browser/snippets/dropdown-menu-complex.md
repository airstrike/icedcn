```rust
use icedxdy::widget::{button, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Complex Menu").style(button::Outline)),
    move |_| {
        menu::group([
            menu::group([
                menu::label("File"),
                menu::item(with_icon(icon::file(), "New File"))
                    .shortcut("⌘N")
                    .into(),
                menu::item(with_icon(icon::folder(), "New Folder"))
                    .shortcut("⇧⌘N")
                    .into(),
                menu::sub(
                    with_icon(icon::folder_open(), "Open Recent"),
                    [
                        menu::group([
                            menu::label("Recent Projects"),
                            menu::item(with_icon(icon::file_code(), "Project Alpha")).into(),
                            menu::item(with_icon(icon::file_code(), "Project Beta")).into(),
                            menu::sub(
                                with_icon(icon::ellipsis(), "More Projects"),
                                [
                                    menu::item(with_icon(icon::file_code(), "Project Gamma"))
                                        .into(),
                                    menu::item(with_icon(icon::file_code(), "Project Delta"))
                                        .into(),
                                ],
                            ),
                        ]),
                        menu::separator(),
                        menu::group([menu::item(with_icon(
                            icon::folder_search(),
                            "Browse...",
                        ))
                        .into()]),
                    ],
                ),
                menu::separator(),
                menu::item(with_icon(icon::save(), "Save"))
                    .shortcut("⌘S")
                    .into(),
                menu::item(with_icon(icon::download(), "Export"))
                    .shortcut("⇧⌘E")
                    .into(),
            ]),
            menu::separator(),
            menu::group([
                menu::label("View"),
                menu::checkbox_item(
                    with_icon(icon::eye(), "Show Sidebar"),
                    state.checked[0],
                    |v| Choice::Checked(0, v),
                )
                .into(),
                menu::checkbox_item(
                    with_icon(icon::panels_top_left(), "Show Status Bar"),
                    state.checked[1],
                    |v| Choice::Checked(1, v),
                )
                .into(),
                menu::sub(
                    with_icon(icon::palette(), "Theme"),
                    [menu::group([
                        menu::label("Appearance"),
                        menu::group([
                            menu::radio_item(
                                with_icon(icon::sun(), "Light"),
                                "light",
                                Some(state.theme),
                                Choice::Theme,
                            )
                            .into(),
                            menu::radio_item(
                                with_icon(icon::moon(), "Dark"),
                                "dark",
                                Some(state.theme),
                                Choice::Theme,
                            )
                            .into(),
                            menu::radio_item(
                                with_icon(icon::monitor(), "System"),
                                "system",
                                Some(state.theme),
                                Choice::Theme,
                            )
                            .into(),
                        ]),
                    ])],
                ),
            ]),
            menu::separator(),
            menu::group([
                menu::label("Account"),
                menu::item(with_icon(icon::user(), "Profile"))
                    .shortcut("⇧⌘P")
                    .into(),
                menu::item(with_icon(icon::credit_card(), "Billing")).into(),
                menu::sub(
                    with_icon(icon::settings(), "Settings"),
                    [
                        menu::group([
                            menu::label("Preferences"),
                            menu::item(with_icon(icon::keyboard(), "Keyboard Shortcuts"))
                                .into(),
                            menu::item(with_icon(icon::languages(), "Language")).into(),
                            menu::sub(
                                with_icon(icon::bell(), "Notifications"),
                                [menu::group([
                                    menu::label("Notification Types"),
                                    menu::checkbox_item(
                                        with_icon(icon::bell(), "Push Notifications"),
                                        state.checked[2],
                                        |v| Choice::Checked(2, v),
                                    )
                                    .into(),
                                    menu::checkbox_item(
                                        with_icon(icon::mail(), "Email Notifications"),
                                        state.checked[0],
                                        |v| Choice::Checked(0, v),
                                    )
                                    .into(),
                                ])],
                            ),
                        ]),
                        menu::separator(),
                        menu::group([menu::item(with_icon(
                            icon::shield(),
                            "Privacy & Security",
                        ))
                        .into()]),
                    ],
                ),
            ]),
            menu::separator(),
            menu::group([
                menu::item(with_icon(icon::circle_question_mark(), "Help & Support")).into(),
                menu::item(with_icon(icon::file_text(), "Documentation")).into(),
            ]),
            menu::separator(),
            menu::group([menu::item(with_icon(icon::log_out(), "Sign Out"))
                .style(menu::Destructive)
                .shortcut("⇧⌘Q")
                .into()]),
        ])
    },
)
.width(176)
```
