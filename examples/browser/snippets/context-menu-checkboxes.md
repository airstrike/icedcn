```rust
use icedxdy::widget::{context_menu, menu};

context_menu(
    |_| area(),
    move |_| {
        menu::group([menu::group([
            menu::checkbox_item("Show Bookmarks Bar", state.checked[0], |v| {
                Choice::Checked(0, v)
            })
            .into(),
            menu::checkbox_item("Show Full URLs", state.checked[1], |v| {
                Choice::Checked(1, v)
            })
            .into(),
            menu::checkbox_item("Show Developer Tools", state.checked[2], |v| {
                Choice::Checked(2, v)
            })
            .into(),
        ])])
    },
)
```
