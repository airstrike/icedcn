```rust
use icedcn::Slot;
use icedcn::widget::breadcrumb;

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoComponents,
}

fn view() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::GoHome),
        breadcrumb::ellipsis(None),
        breadcrumb::link("Components", Message::GoComponents),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}
```
