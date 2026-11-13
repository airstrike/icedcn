```rust
use icedxdy::widget::{breadcrumb, button, dropdown_menu, menu};
use icedxdy::Size;

breadcrumb([
    breadcrumb::link("Home", Message::GoHome),
    breadcrumb::item(
        dropdown_menu(
            |m| {
                m.trigger(
                    button(icon::ellipsis().size(16))
                        .size(Size::Sm)
                        .square()
                        .style(button::Ghost)
                        .translate(false),
                )
            },
            |_| {
                menu::group([
                    menu::item("Documentation").into(),
                    menu::item("Themes").into(),
                    menu::item("GitHub").into(),
                ])
            },
        )
        .align(dropdown_menu::Align::Start),
    ),
    breadcrumb::link("Components", Message::GoComponents),
    breadcrumb::page("Breadcrumb"),
])
.separator(|| icon::chevron_right().size(14).into())
```
