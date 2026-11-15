```rust
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::{container, text};

container(text::inline_code("widget::alert_dialog::AlertDialog"))
    .padding([2, 5])
    .style(|t| Style {
        background: Some(t.muted.base.into()),
        border: Border { radius: 4.0.into(), ..Default::default() },
        ..Default::default()
    })
```
