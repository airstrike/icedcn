```rust
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Widget};
use icedcn::widget::button::group;
use icedcn::widget::{button, button_group, column, dropdown_menu, menu, row};
use icedcn::{Slot, Theme};
use crate::Message;

mod icon {
use iced::widget::svg;
use icedcn::Theme;
pub fn arrow_left<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m12 19-7-7 7-7" /> <path d="M19 12H5" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn ellipsis<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="12" cy="12" r="1" /> <circle cx="19" cy="12" r="1" /> <circle cx="5" cy="12" r="1" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn mail_check<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" /> <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" /> <path d="m16 19 2 2 4-4" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn archive<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <rect width="20" height="5" x="2" y="3" rx="1" /> <path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" /> <path d="M10 12h4" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn clock<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="12" cy="12" r="10" /> <polyline points="12 6 12 12 16 14" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn calendar_plus<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 2v4" /> <path d="M16 2v4" /> <path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" /> <path d="M3 10h18" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn list_filter<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M7 12h10" /> <path d="M10 18h4" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn tag<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z" /> <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn trash_2<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /> <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" /> <line x1="10" x2="10" y1="11" y2="17" /> <line x1="14" x2="14" y1="11" y2="17" /> </svg>"#.as_bytes()))
        .width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.destructive.base.into()) })
}
}
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
            button(icon::arrow_left().boxed()).square().style(button::Outline),
            button_group![button("Archive").style(button::Outline), button("Report").style(button::Outline)],
            dropdown_menu(|&m| button_group![
                button("Snooze").style(button::Outline),
                group::decorated(button(icon::ellipsis().boxed()).style(button::Outline).square(), move |b| m.trigger(b))
            ].into(), move |_| column![
                menu::item(row![icon::mail_check().boxed(), "Mark as Read"].spacing(8)),
                menu::item(row![icon::archive().boxed(), "Archive"].spacing(8)),
                menu::separator(),
                menu::item(row![icon::clock().boxed(), "Snooze"].spacing(8)),
                menu::item(row![icon::calendar_plus().boxed(), "Add to Calendar"].spacing(8)),
                menu::item(row![icon::list_filter().boxed(), "Add to List"].spacing(8)),
                menu::sub(row![icon::tag().boxed(), "Label As..."].spacing(8), ["Personal", "Work", "Other"].map(|label| menu::radio_item(label, label, Some(selected), |label| label).into())),
                menu::separator(),
                menu::item(row![icon::trash_2().boxed(), "Trash"].spacing(8)).style(menu::Destructive)
            ].width(Fill).into()).align(dropdown_menu::Align::End).width(160)
        ].spacing(8).align_y(iced::Center))
    }
}
pub(super) fn demo() -> Slot<'static, Message> { Slot::from(component(MailActions).boxed()) }

```
