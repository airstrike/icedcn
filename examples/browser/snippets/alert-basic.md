```rust
use crate::icon;
use icedcn::Slot;
use icedcn::widget::{alert, text};

fn account_updated<M: 'static>(wrapping: text::Wrapping) -> Slot<'static, M> {
    alert()
        .icon(icon::circle_check().size(16).line_height(1.0))
        .title("Account updated successfully")
        .description(
            "Your profile information has been saved. Changes will be reflected immediately.",
        )
        .description_wrapping(wrapping)
        .max_width(448)
        .into()
}
```
