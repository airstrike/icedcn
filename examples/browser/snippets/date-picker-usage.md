## Usage

Keep the selection in the widget, or use `.on_value_change(...)` to receive each change. Pass `.value(...)` when the application controls the selection.

```rust
use icedcn::widget::{calendar::Selection, date_picker};
use icedcn::Slot;

pub fn view(selected: Option<Selection>) -> Slot<'static, Option<Selection>> {
    date_picker().value(selected).on_value_change(|next| next).into()
}
```

Store the returned selection in your application’s update function.
