```rust
use icedcn::widget::{alert, button};
use icedcn::{Size, Slot};

fn dark_mode<'a, M: Clone + 'a>(enable: Option<M>) -> Slot<'a, M> {
    alert()
        .title("Dark mode is now available")
        .description("Enable it under your profile settings to get started.")
        .action(button("Enable").size(Size::Xs).on_press_maybe(enable))
        .max_width(448)
        .into()
}
```

Pass `Some(Message::Enable)` to handle the action in your application. The preview passes `None`, so pressing Enable leaves the callout visible.
