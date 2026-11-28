```rust
use crate::icon;
use iced::widget::{Text, responsive};
use iced::{Fill, Shrink, Widget};
use icedcn::widget::{Row, button, container, dropdown_menu, menu};
use icedcn::{View, Theme};

pub fn view<'a>(selected: &'static str) -> View<'a, &'static str> {
    responsive(move |area| {
        let outline = |label| button(label).style(button::Outline);
        let square = |glyph: Text<'a, Theme>| {
            button(glyph.size(16).line_height(1.0))
                .square()
                .style(button::Outline)
        };
        let mut groups = Row::new().spacing(8).align_y(iced::Center);
        if area.width >= 320. {
            groups = groups.push(button::group![square(icon::arrow_left())].into());
        }
        let more = dropdown_menu(
            move |m| {
                button::group![
                    outline("Snooze"),
                    m.trigger(square(icon::ellipsis()).has_popup(true)),
                ]
                .into()
            },
            move |_| {
                let caption = |glyph: Text<'a, Theme>, label| {
                    menu::caption(glyph.size(16).line_height(1.0), label)
                };
                let item = |glyph, label| menu::item(caption(glyph, label));
                let labels = ["Personal", "Work", "Other"].map(|label| {
                    menu::radio_item(label, label, Some(selected), |value| value).into()
                });
                menu::group![
                    item(icon::mail_check(), "Mark as Read"),
                    item(icon::archive(), "Archive"),
                    menu::separator(),
                    item(icon::clock(), "Snooze"),
                    item(icon::calendar_plus(), "Add to Calendar"),
                    item(icon::list_filter(), "Add to List"),
                    menu::sub(caption(icon::tag(), "Label As..."), labels),
                    menu::separator(),
                    item(icon::trash_2(), "Trash").style(menu::Destructive),
                ]
            },
        )
        .align(dropdown_menu::Align::End)
        .width(160);
        View::from(
            container(
                groups
                    .push(button::group![outline("Archive"), outline("Report")].into())
                    .push(more.into()),
            )
            .center_x(Fill),
        )
        .compose()
    })
    .height(Shrink)
    .boxed()
    .into()
}
```

Start with `"Personal"`. Store the returned label in your application’s `update` and pass it back to `view`.
