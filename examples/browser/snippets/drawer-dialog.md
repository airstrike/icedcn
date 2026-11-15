```rust
use icedcn::widget::{button, column, dialog, drawer};

drawer(
    |d| d.trigger(button("Edit Profile").style(button::Outline)),
    |d| {
        if d.is_mobile() {
            column![
                drawer::header("Edit profile", description),
                form(d),
                drawer::footer([d.close(button("Cancel").style(button::Outline))]),
            ]
            .into()
        } else {
            column![dialog::header("Edit profile", description), form(d)]
                .spacing(16)
                .into()
        }
    },
)
.desktop_dialog(425.0)
```
