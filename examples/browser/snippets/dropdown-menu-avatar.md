```rust
use crate::icon;
use iced::widget::Text;
use icedcn::widget::{avatar, button, dropdown_menu, menu, menu_group};
use icedcn::{Slot, Theme};

pub fn view<'a, Message: Clone + 'static>(photo: iced::widget::image::Handle) -> Slot<'a, Message> {
    let caption = |glyph: Text<'a, Theme>, label: &'a str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let item = move |glyph, label| menu::item(caption(glyph, label));
    dropdown_menu(
        move |m| {
            m.trigger(
                button(avatar(Some(photo.clone())).fallback("LR"))
                    .square()
                    .rounded()
                    .translate(false)
                    .style(button::Ghost),
            )
        },
        move |_| {
            menu_group![
                item(icon::badge_check(), "Account"),
                item(icon::credit_card(), "Billing"),
                item(icon::bell(), "Notifications"),
                menu::separator(),
                item(icon::log_out(), "Sign Out"),
            ]
        },
    )
    .align(dropdown_menu::Align::End)
    .into()
}
```
