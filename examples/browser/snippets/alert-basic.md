```rust
use crate::icon;
use icedcn::View;
use icedcn::widget::alert;

fn account_updated<'a, M: 'a>() -> View<'a, M> {
    alert()
        .icon(icon::circle_check().size(16).line_height(1.0))
        .title("Account updated successfully")
        .description(
            "Your profile information has been saved. Changes will be reflected immediately.",
        )
        .max_width(448)
        .into()
}
```
