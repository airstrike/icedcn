```rust
use iced::Widget;
use iced::widget::svg;
use icedcn::widget::breadcrumb;
use icedcn::{Slot, Theme};

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoComponents,
}

fn dot() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12.1" cy="12.1" r="1"/></svg>"#))
        .width(14)
        .height(14)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.muted.foreground.into()) })
        .boxed()
        .into()
}

fn view() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::GoHome),
        breadcrumb::link("Components", Message::GoComponents),
        breadcrumb::page("Breadcrumb"),
    ])
    .separator(dot)
    .into()
}
```
