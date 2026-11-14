```rust
use icedcn::widget::{context_menu, menu};

context_menu(
    |_| area(),
    move |_| {
        menu::group([
            menu::group([
                menu::label("People"),
                menu::group([
                    menu::radio_item(
                        "Pedro Duarte",
                        "pedro",
                        Some(state.person),
                        Choice::Person,
                    )
                    .into(),
                    menu::radio_item("Colm Tuite", "colm", Some(state.person), Choice::Person)
                        .into(),
                ]),
            ]),
            menu::separator(),
            menu::group([
                menu::label("Theme"),
                menu::group([
                    menu::radio_item("Light", "light", Some(state.theme), Choice::Theme).into(),
                    menu::radio_item("Dark", "dark", Some(state.theme), Choice::Theme).into(),
                    menu::radio_item("System", "system", Some(state.theme), Choice::Theme)
                        .into(),
                ]),
            ]),
        ])
    },
)
```
