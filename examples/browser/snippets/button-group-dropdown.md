```rust
use crate::Message;
use iced::{Fill, Widget};
use icedcn::Slot;
use icedcn::widget::{button, button_group, column, dropdown_menu, menu, row};

pub fn view() -> Slot<'static, Message> {
    dropdown_menu(
        |&m| {
            button_group![
                button("Follow").style(button::Outline),
                m.trigger(
                    button(icon::chevron_down().boxed())
                        .style(button::Outline)
                        .padding(iced::Padding::ZERO.left(8).right(10))
                        .has_popup(true)
                )
            ]
            .into()
        },
        |_| {
            column![
                menu::item(row![icon::volume_off().boxed(), "Mute Conversation"].spacing(8)),
                menu::item(row![icon::check().boxed(), "Mark as Read"].spacing(8)),
                menu::item(row![icon::triangle_alert().boxed(), "Report Conversation"].spacing(8)),
                menu::item(row![icon::user_round_x().boxed(), "Block User"].spacing(8)),
                menu::item(row![icon::share().boxed(), "Share Conversation"].spacing(8)),
                menu::item(row![icon::copy().boxed(), "Copy Conversation"].spacing(8)),
                menu::separator(),
                menu::item(row![icon::trash().boxed(), "Delete Conversation"].spacing(8))
                    .style(menu::Destructive)
            ]
            .width(Fill)
            .into()
        },
    )
    .align(dropdown_menu::Align::End)
    .width(176)
    .into()
}

mod icon {
    use iced::widget::svg;
    use icedcn::Theme;
    pub fn check<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6 9 17l-5-5" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn chevron_down<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m6 9 6 6 6-6" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn copy<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <rect width="14" height="14" x="8" y="8" rx="2" ry="2" /> <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn share<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" /> <polyline points="16 6 12 2 8 6" /> <line x1="12" x2="12" y1="2" y2="15" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn trash<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /> <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.destructive.base.into()) })
    }
    pub fn triangle_alert<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" /> <path d="M12 9v4" /> <path d="M12 17h.01" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn user_round_x<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 21a8 8 0 0 1 11.873-7" /> <circle cx="10" cy="8" r="5" /> <path d="m17 17 5 5" /> <path d="m22 17-5 5" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
    pub fn volume_off<'a>() -> svg::Svg<'a, Theme> {
        svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 9a5 5 0 0 1 .95 2.293" /> <path d="M19.364 5.636a9 9 0 0 1 1.889 9.96" /> <path d="m2 2 20 20" /> <path d="m7 7-.587.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298V11" /> <path d="M9.828 4.172A.686.686 0 0 1 11 4.657v.686" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
    }
}
```
