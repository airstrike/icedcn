```rust
use crate::icon;
use iced::widget::Text;
use icedcn::widget::{button, dropdown_menu, menu, menu_group};
use icedcn::{Slot, Theme};

pub fn view<Message: Clone + 'static>() -> Slot<'static, Message> {
    let caption = |glyph: Text<'static, Theme>, label: &'static str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let item = move |glyph, label| menu::item(caption(glyph, label));
    dropdown_menu(
        |m| m.trigger(button("Actions").style(button::Outline)),
        move |_| menu_group![
            item(icon::pencil(), "Edit"), item(icon::share(), "Share"),
            menu::separator(), item(icon::trash_2(), "Delete").style(menu::Destructive),
        ],
    ).into()
}
```
