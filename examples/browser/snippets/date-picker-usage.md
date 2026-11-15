## Usage

Keep the selection in the widget, or use `.on_value_change(...)` to receive each change. Pass `.value(...)` when the application controls the selection.

```rust
use icedcn::Slot;
use icedcn::widget::{calendar::Selection, date_picker};

pub fn view<'a>(selected: Option<Selection>) -> Slot<'a, Option<Selection>> {
    date_picker()
        .value(selected)
        .on_value_change(|next| next)
        .into()
}
```

Store the returned selection in your application’s update function.
