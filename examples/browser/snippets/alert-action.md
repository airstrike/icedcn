```rust
use icedcn::widget::{alert, button, text};
use icedcn::{Size, Slot};

fn dark_mode<M: Clone + 'static>(wrapping: text::Wrapping, enable: Option<M>) -> Slot<'static, M> {
    alert()
        .title("Dark mode is now available")
        .description("Enable it under your profile settings to get started.")
        .description_wrapping(wrapping)
        .action(button("Enable").size(Size::Xs).on_press_maybe(enable))
        .max_width(448)
        .into()
}
```

Pass `Some(Message::Enable)` to handle the action in your application. The preview passes `None`, so pressing Enable leaves the callout visible.
