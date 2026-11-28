## Usage

Supply the current trail in reading order. Handle `Message::GoHome` and `Message::GoComponents` in your application update function to change the displayed route.

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
        breadcrumb::link("Components", Message::GoComponents),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}
```
