```rust
use iced::Widget;
use iced::widget::button::{Style, StyleFn};
use iced::widget::svg;
use icedxdy::Weight;
use icedxdy::widget::{breadcrumb, button, dropdown_menu, menu, row};
use icedxdy::{Slot, Theme};

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoComponents,
}

fn down() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#))
        .width(14)
        .height(14)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.muted.foreground.into()) })
        .boxed()
        .into()
}

fn dot() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12.1" cy="12.1" r="1"/></svg>"#))
        .width(14)
        .height(14)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.muted.foreground.into()) })
        .boxed()
        .into()
}

fn destinations(_: &dropdown_menu::Handle) -> Slot<'static, Message> {
    menu::group([
        menu::item("Documentation").into(),
        menu::item("Themes").into(),
        menu::item("GitHub").into(),
    ])
}

fn trigger_style() -> StyleFn<'static, Theme> {
    Box::new(|theme, _| Style {
        text_color: theme.muted.foreground.into(),
        ..Default::default()
    })
}

fn view() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::GoHome),
        breadcrumb::item(
            dropdown_menu(
                |m| {
                    m.trigger(
                        button(row!["Components", down()].spacing(4))
                            .height(20)
                            .padding(0)
                            .weight(Weight::Normal)
                            .translate(false)
                            .focus_ring(2, |theme| theme.neutral.foreground)
                            .style(trigger_style()),
                    )
                },
                destinations,
            )
            .align(dropdown_menu::Align::Start),
        ),
        breadcrumb::page("Breadcrumb"),
    ])
    .separator(dot)
    .into()
}
```
