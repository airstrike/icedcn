```rust
use crate::icon;
use iced::widget::Text;
use icedcn::widget::{button, dropdown_menu, menu};
use icedcn::{Slot, Theme};

pub fn view<'a>(checked: [bool; 3]) -> Slot<'a, [bool; 3]> {
    let caption = |glyph: Text<'a, Theme>, label: &'a str| {
        menu::caption(glyph.size(16).line_height(1.0), label)
    };
    let toggle = move |i: usize, value| {
        let mut next = checked;
        next[i] = value;
        next
    };
    dropdown_menu(
        |m| m.trigger(button("Notifications").style(button::Outline)),
        move |_| {
            menu::group![
                menu::label("Notification Preferences"),
                menu::checkbox_item(
                    caption(icon::mail(), "Email notifications"),
                    checked[0],
                    |v| toggle(0, v)
                ),
                menu::checkbox_item(
                    caption(icon::message_square(), "SMS notifications"),
                    checked[1],
                    |v| toggle(1, v)
                ),
                menu::checkbox_item(
                    caption(icon::bell(), "Push notifications"),
                    checked[2],
                    |v| toggle(2, v)
                ),
            ]
        },
    )
    .width(192)
    .into()
}
```

Initialize the application value to `[true, false, true]`. Keep the value this menu publishes when an item is selected.
