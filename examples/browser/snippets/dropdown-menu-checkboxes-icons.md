```rust
use icedxdy::widget::{button, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Notifications").style(button::Outline)),
    move |_| {
        menu::group([menu::group([
            menu::label("Notification Preferences"),
            menu::checkbox_item(
                with_icon(icon::mail(), "Email notifications"),
                state.checked[0],
                |v| Choice::Checked(0, v),
            )
            .into(),
            menu::checkbox_item(
                with_icon(icon::message_square(), "SMS notifications"),
                state.checked[1],
                |v| Choice::Checked(1, v),
            )
            .into(),
            menu::checkbox_item(
                with_icon(icon::bell(), "Push notifications"),
                state.checked[2],
                |v| Choice::Checked(2, v),
            )
            .into(),
        ])])
    },
)
.width(192)
```
