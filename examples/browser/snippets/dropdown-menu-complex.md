```rust
use crate::icon;
use iced::widget::Text;
use icedcn::widget::{button, dropdown_menu, menu, menu_group, menu_sub};
use icedcn::{Slot, Theme};

#[derive(Debug, Clone, Copy)]
pub struct Preferences {
    pub email: bool,
    pub sms: bool,
    pub push: bool,
    pub theme: &'static str,
}

pub fn view<'a>(p: Preferences) -> Slot<'a, Preferences> {
    let caption = |glyph: Text<'a, Theme>, label: &'a str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let item = move |glyph, label| menu::item(caption(glyph, label));
    dropdown_menu(
        |m| m.trigger(button("Complex Menu").style(button::Outline)),
        move |_| {
            let projects = menu_sub![
                caption(icon::ellipsis(), "More Projects"),
                item(icon::file_code(), "Project Gamma"),
                item(icon::file_code(), "Project Delta")
            ];
            let recent = menu_sub![
                caption(icon::folder_open(), "Open Recent"),
                menu::label("Recent Projects"),
                item(icon::file_code(), "Project Alpha"),
                item(icon::file_code(), "Project Beta"),
                projects,
                menu::separator(),
                item(icon::folder_search(), "Browse...")
            ];
            let theme = |g, label, value| {
                menu::radio_item(caption(g, label), value, Some(p.theme), |theme| {
                    Preferences { theme, ..p }
                })
            };
            let appearance = menu_sub![
                caption(icon::palette(), "Theme"),
                menu::label("Appearance"),
                theme(icon::sun(), "Light", "light"),
                theme(icon::moon(), "Dark", "dark"),
                theme(icon::monitor(), "System", "system")
            ];
            let notifications = menu_sub![
                caption(icon::bell(), "Notifications"),
                menu::label("Notification Types"),
                menu::checkbox_item(
                    caption(icon::bell(), "Push Notifications"),
                    p.push,
                    |push| Preferences { push, ..p }
                ),
                menu::checkbox_item(
                    caption(icon::mail(), "Email Notifications"),
                    p.email,
                    |email| Preferences { email, ..p }
                )
            ];
            let settings = menu_sub![
                caption(icon::settings(), "Settings"),
                menu::label("Preferences"),
                item(icon::keyboard(), "Keyboard Shortcuts"),
                item(icon::languages(), "Language"),
                notifications,
                menu::separator(),
                item(icon::shield(), "Privacy & Security")
            ];
            menu_group![
                menu::label("File"),
                item(icon::file(), "New File").shortcut("⌘N"),
                item(icon::folder(), "New Folder").shortcut("⇧⌘N"),
                recent,
                menu::separator(),
                item(icon::save(), "Save").shortcut("⌘S"),
                item(icon::download(), "Export").shortcut("⇧⌘E"),
                menu::separator(),
                menu::label("View"),
                menu::checkbox_item(caption(icon::eye(), "Show Sidebar"), p.email, |email| {
                    Preferences { email, ..p }
                }),
                menu::checkbox_item(
                    caption(icon::panels_top_left(), "Show Status Bar"),
                    p.sms,
                    |sms| Preferences { sms, ..p }
                ),
                appearance,
                menu::separator(),
                menu::label("Account"),
                item(icon::user(), "Profile").shortcut("⇧⌘P"),
                item(icon::credit_card(), "Billing"),
                settings,
                menu::separator(),
                item(icon::circle_question_mark(), "Help & Support"),
                item(icon::file_text(), "Documentation"),
                menu::separator(),
                item(icon::log_out(), "Sign Out")
                    .shortcut("⇧⌘Q")
                    .style(menu::Destructive),
            ]
        },
    )
    .width(176)
    .into()
}
```

Pass the current preferences into `view` and keep the returned message as the next preferences in `update`.
