```rust
use crate::icon;
use iced::widget::Text;
use icedcn::widget::{button, dropdown_menu, menu};
use icedcn::{View, Theme};

pub fn view<'a>() -> View<'a> {
    dropdown_menu(
        |&m| {
            button::group![
                button("Follow").style(button::Outline),
                m.trigger(
                    button(icon::chevron_down().size(16).line_height(1.0))
                        .style(button::Outline)
                        .padding(iced::Padding::ZERO.left(8).right(10))
                        .has_popup(true)
                )
            ]
            .into()
        },
        |_| {
            let item = |glyph: Text<'a, Theme>, label| {
                menu::item(menu::caption(glyph.size(16).line_height(1.0), label))
            };
            menu::group![
                item(icon::volume_off(), "Mute Conversation"),
                item(icon::check(), "Mark as Read"),
                item(icon::triangle_alert(), "Report Conversation"),
                item(icon::user_round_x(), "Block User"),
                item(icon::share(), "Share Conversation"),
                item(icon::copy(), "Copy Conversation"),
                menu::separator(),
                item(icon::trash(), "Delete Conversation").style(menu::Destructive),
            ]
        },
    )
    .align(dropdown_menu::Align::End)
    .width(176)
    .into()
}
```
