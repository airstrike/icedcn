```rust
use iced::Fill;
use icedcn::widget::{button, column, dialog, field, input, text};

dialog(
    |d| d.trigger(button("Open Dialog").style(button::Outline)),
    |_| {
        let profile_field = |label, value| {
            field(input("").default_value(value).width(Fill))
                .label(text::label(label).height(14))
                .width(Fill)
                .into()
        };
        column![
            dialog::header(
                "Edit profile",
                "Make changes to your profile here. Click save when you're done.",
            ),
            field::group([
                profile_field("Name", "Pedro Duarte"),
                profile_field("Username", "@peduarte"),
            ]),
        ]
        .spacing(16)
        .width(Fill)
        .into()
    },
)
.footer(|d| {
    [
        d.close(button("Cancel").style(button::Outline)),
        button("Save changes").into(),
    ]
})
```

For windows narrower than 768 pixels, the preview sets `.text_size(16)` on its inputs.
