```rust
use icedxdy::widget::{menu, menubar};

menubar([
    menubar::menu("Profiles", {
        let user = state.user;
        move || {
            menu::group([
                menu::group([
                    menu::radio_item("Andy", "andy", Some(user), Choice::User).into(),
                    menu::radio_item("Benoit", "benoit", Some(user), Choice::User).into(),
                    menu::radio_item("Luis", "luis", Some(user), Choice::User).into(),
                ]),
                menu::separator(),
                menu::item("Edit...").inset(true).into(),
                menu::item("Add Profile...").inset(true).into(),
            ])
        }
    }),
    menubar::menu("Theme", {
        let theme = state.theme;
        move || {
            menu::group([menu::group([
                menu::radio_item("Light", "light", Some(theme), Choice::Theme).into(),
                menu::radio_item("Dark", "dark", Some(theme), Choice::Theme).into(),
                menu::radio_item("System", "system", Some(theme), Choice::Theme).into(),
            ])])
        }
    }),
])
.width(288)
```
