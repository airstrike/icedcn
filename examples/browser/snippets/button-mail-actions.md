```rust
use crate::icon;
use iced::{Fill, widget::Text};
use icedcn::widget::{button, button_group, column, dropdown_menu, menu, row};
use icedcn::{Slot, Theme};

#[derive(Clone, Debug)]
pub enum Message {
    LabelSelected(&'static str),
}

pub fn view<'a>(window_width: f32, selected: &'static str) -> Slot<'a, Message> {
    let outline = |label| button(label).style(button::Outline);
    let caption =
        |icon: Text<'static, Theme>, label: &'static str| row![icon.size(16), label].spacing(8);
    let actions = dropdown_menu(
        move |m| {
            button_group![
                outline("Snooze"),
                m.trigger(
                    button(icon::ellipsis().size(16))
                        .square()
                        .style(button::Outline)
                        .has_popup(true)
                ),
            ]
            .into()
        },
        move |_| {
            column![
                menu::item(caption(icon::mail_check(), "Mark as Read")),
                menu::item(caption(icon::archive(), "Archive")),
                menu::separator(),
                menu::item(caption(icon::clock(), "Snooze")),
                menu::item(caption(icon::calendar_plus(), "Add to Calendar")),
                menu::item(caption(icon::list_filter(), "Add to List")),
                menu::sub(
                    caption(icon::tag(), "Label As..."),
                    ["Personal", "Work", "Other"].map(|label| {
                        menu::radio_item(label, label, Some(selected), Message::LabelSelected)
                            .into()
                    })
                ),
                menu::separator(),
                menu::item(caption(icon::trash_2(), "Trash")).style(menu::Destructive),
            ]
            .width(Fill)
            .into()
        },
    )
    .align(dropdown_menu::Align::End)
    .width(160);

    let mut groups = row![];
    if window_width >= 640.0 {
        groups = groups.push(
            button_group![
                button(icon::arrow_left().size(16))
                    .square()
                    .style(button::Outline),
            ]
            .into(),
        );
    }
    groups
        .push(button_group![outline("Archive"), outline("Report")].into())
        .push(actions.into())
        .spacing(8)
        .into()
}
```
