```rust
use icedcn::widget::{attachment, dialog};

dialog(
    |d| {
        let opener = *d;
        attachment("research-summary.pdf")
            .trigger(move |target| opener.trigger(attachment::trigger(target)))
            .into()
    },
    |_| dialog::header("research-summary.pdf", "Attachment preview"),
)
```
