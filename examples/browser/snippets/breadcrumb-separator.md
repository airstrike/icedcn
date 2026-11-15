```rust
use crate::icon;
use icedcn::Slot;
use icedcn::theme::text::muted;
use icedcn::widget::breadcrumb;

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoComponents,
}

fn view<'a>() -> Slot<'a, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::GoHome),
        breadcrumb::link("Components", Message::GoComponents),
        breadcrumb::page("Breadcrumb"),
    ])
    .separator(|| {
        icon::dot()
            .size(14)
            .line_height(1.0)
            .style(|theme| muted(theme).into())
            .into()
    })
    .into()
}
```
