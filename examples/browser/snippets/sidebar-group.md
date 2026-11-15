```rust
use icedcn::Slot;
use icedcn::widget::{collapsible, sidebar};

pub fn help<'a>() -> Slot<'a, ()> {
    let group = collapsible(|c| {
        let links = ["Support", "Feedback"].map(|name| sidebar::menu_button(name).into());
        sidebar::group([
            c.trigger(sidebar::menu_button("Help")),
            c.content(sidebar::menu(links)),
        ]).into()
    }).width(iced::Fill).default_open(true);

    sidebar::floating(group).width(sidebar::WIDTH).height(iced::Fill).into()
}
```
