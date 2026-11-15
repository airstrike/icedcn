```rust
use icedcn::Slot;
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
    .into()
}
```
