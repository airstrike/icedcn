### Label icon-only actions

Name the operation and the file, such as “Remove sales-dashboard.pdf”. The preview shows a tooltip on icon-only controls. For a visible label, pass a text button to `.action(...)`.

```rust
use icedcn::widget::{attachment, button};

attachment("sales-dashboard.pdf").action(
    button("Remove sales-dashboard.pdf")
        .style(button::Ghost)
        .on_press(Message::Remove("sales-dashboard.pdf")),
)
```
