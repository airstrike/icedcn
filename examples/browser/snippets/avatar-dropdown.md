```rust
use iced::widget::image;
use icedxdy::Theme;
use icedxdy::widget::avatar;
use icedxdy::widget::{button, column, dropdown_menu, menu};

fn preview(shadcn: image::Handle) -> iced::Element<'static, (), Theme> {
    dropdown_menu(
        move |menu| {
            menu.trigger(
                button(avatar(Some(shadcn.clone())).fallback("CN"))
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
