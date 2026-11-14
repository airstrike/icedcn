```rust
use crate::icon;
use iced::widget::Text;
use icedcn::{Slot, Theme};
use icedcn::widget::{Row, button, button_group, dropdown_menu, menu, menu_group};

pub fn view(window_width: f32, selected: &'static str) -> Slot<'static, &'static str> {
    let outline = |label| button(label).style(button::Outline);
    let square = |glyph: Text<'static, Theme>| {
        button(glyph.size(16).line_height(1.0)).square().style(button::Outline)
    };
    let mut groups = Row::new().spacing(8).align_y(iced::Center);
    if window_width >= 640. {
        groups = groups.push(button_group![square(icon::arrow_left())].into());
    }
    let more = dropdown_menu(
        move |m| button_group![
            outline("Snooze"), m.trigger(square(icon::ellipsis()).has_popup(true)),
        ].into(),
        move |_| {
            let caption = |glyph: Text<'static, Theme>, label| {
                menu::caption(glyph.size(16).line_height(1.0), label)
            };
            let item = |glyph, label| menu::item(caption(glyph, label));
            let labels = ["Personal", "Work", "Other"].map(|label| {
                menu::radio_item(label, label, Some(selected), |value| value).into()
            });
            menu_group![
                item(icon::mail_check(), "Mark as Read"), item(icon::archive(), "Archive"),
                menu::separator(), item(icon::clock(), "Snooze"),
                item(icon::calendar_plus(), "Add to Calendar"), item(icon::list_filter(), "Add to List"),
                menu::sub(caption(icon::tag(), "Label As..."), labels), menu::separator(),
                item(icon::trash_2(), "Trash").style(menu::Destructive),
            ]
        },
    ).align(dropdown_menu::Align::End).width(160);
    groups.push(button_group![outline("Archive"), outline("Report")].into()).push(more.into()).into()
}
```

Start with `"Personal"`. Store the returned label in your application’s `update` and pass it back to `view`.
