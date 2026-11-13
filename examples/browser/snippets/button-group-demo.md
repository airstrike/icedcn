```rust
struct MailActions;
impl<'a> Component<'a, Message, Theme> for MailActions {
    type State = Option<&'static str>;
    type Event = &'static str;
    fn update(&self, label: &mut Self::State, value: Self::Event, _: &iced::Renderer) -> Option<Message> {
        *label = Some(value);
        None
    }
    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let selected = state.unwrap_or("Personal");
        Element::from(row![
            button(icon::arrow_left()).square().style(button::Outline),
            button_group![button("Archive").style(button::Outline), button("Report").style(button::Outline)],
            dropdown_menu(|&m| button_group![
                button("Snooze").style(button::Outline),
                group::decorated(button(icon::ellipsis()).style(button::Outline).square(), move |b| m.trigger(b))
            ].into(), move |_| column![
                menu::item(row![icon::mail_check().size(16), "Mark as Read"].spacing(8)),
                menu::item(row![icon::archive().size(16), "Archive"].spacing(8)),
                menu::separator(),
                menu::item(row![icon::clock().size(16), "Snooze"].spacing(8)),
                menu::item(row![icon::calendar_plus().size(16), "Add to Calendar"].spacing(8)),
                menu::item(row![icon::list_filter().size(16), "Add to List"].spacing(8)),
                menu::sub(row![icon::tag(), "Label As..."].spacing(8), ["Personal", "Work", "Other"].map(|label| menu::radio_item(label, label, Some(selected), |label| label).into())),
                menu::separator(),
                menu::item(row![icon::trash_2().size(16), "Trash"].spacing(8)).style(menu::Destructive)
            ].width(Fill).into()).align(dropdown_menu::Align::End).width(160)
        ].spacing(8).align_y(iced::Center))
    }
}
pub(super) fn demo() -> Slot<'static, Message> { Slot::from(component(MailActions).boxed()) }

```
