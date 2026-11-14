```rust
use iced::Widget;
use iced::widget::svg;
use icedxdy::Size;
use icedxdy::widget::{breadcrumb, button, dropdown_menu, menu};
use icedxdy::{Slot, Theme};

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoComponents,
}

fn more() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>"#))
        .width(16)
        .height(16)
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

fn view() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::GoHome),
        breadcrumb::item(
            dropdown_menu(
                |m| {
                    m.trigger(
                        button(more())
                            .size(Size::Sm)
                            .square()
                            .style(button::Ghost)
                            .translate(false),
                    )
                },
                destinations,
            )
            .align(dropdown_menu::Align::Start)
            .width(144),
        ),
        breadcrumb::link("Components", Message::GoComponents),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}
```
