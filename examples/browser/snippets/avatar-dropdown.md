```rust
use iced::widget::image;
use icedcn::Theme;
use icedcn::widget::avatar;
use icedcn::widget::{button, column, dropdown_menu, menu};

fn preview(portrait: image::Handle) -> iced::Element<'static, (), Theme> {
    dropdown_menu(
        move |menu| {
            menu.trigger(
                button(avatar(Some(portrait.clone())).fallback("AC"))
                    .style(button::Ghost)
                    .square()
                    .rounded(),
            )
        },
        |_| {
            column![
                menu::item("Profile"),
                menu::item("Billing"),
                menu::item("Settings"),
                menu::separator(),
                menu::item("Log out").style(menu::Destructive),
            ]
            .width(iced::Fill)
            .into()
        },
    )
    .width(128)
    .into()
}
```
