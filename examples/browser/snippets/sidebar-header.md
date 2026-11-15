```rust
use icedcn::Slot;
use icedcn::widget::{dropdown_menu, menu, sidebar};

pub fn workspace_header<'a>() -> Slot<'a, ()> {
    let switcher = dropdown_menu(
        |m| m.trigger(sidebar::menu_button("Select Workspace")),
        |_| menu::group([menu::item("Acme Inc").into()]),
    ).align(dropdown_menu::Align::Start).width(240);

    sidebar::floating(sidebar::header([switcher.into()]))
        .width(sidebar::WIDTH).height(iced::Fill).into()
}
```
