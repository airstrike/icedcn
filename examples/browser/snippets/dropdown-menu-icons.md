```rust
use crate::icon;
use iced::widget::Text;
use icedcn::widget::{button, dropdown_menu, menu};
use icedcn::{Slot, Theme};

pub fn view<'a, Message: Clone + 'static>() -> Slot<'a, Message> {
    let caption = |glyph: Text<'a, Theme>, label: &'a str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let item = move |glyph, label| menu::item(caption(glyph, label));
    dropdown_menu(
        |m| m.trigger(button("Open").style(button::Outline)),
        move |_| {
            menu::group![
                item(icon::user(), "Profile"),
                item(icon::credit_card(), "Billing"),
                item(icon::settings(), "Settings"),
                menu::separator(),
                item(icon::log_out(), "Log out").style(menu::Destructive),
            ]
        },
    )
    .into()
}
```
