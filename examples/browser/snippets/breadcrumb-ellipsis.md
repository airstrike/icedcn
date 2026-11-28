```rust
use icedcn::View;
use icedcn::widget::breadcrumb;

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoComponents,
}

fn view<'a>() -> View<'a, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::GoHome),
        breadcrumb::ellipsis(None),
        breadcrumb::link("Components", Message::GoComponents),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}
```
