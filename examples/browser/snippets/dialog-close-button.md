```rust
use iced::{Fill, alignment::Horizontal};
use icedcn::widget::{button, column, dialog, input};

dialog(
    |d| d.trigger(button("Share").style(button::Outline)),
    |_| {
        column![
            dialog::header("Share link", "Anyone who has this link will be able to view this."),
            input("").value("https://github.com/airstrike/icedcn").width(Fill),
        ]
        .spacing(16)
        .width(Fill)
        .into()
    },
)
.max_width(448.)
.footer(|d| [d.close(button("Close"))])
.footer_align_x(Horizontal::Left)
```

For windows narrower than 768 pixels, the preview sets `.text_size(16)` on its inputs.
