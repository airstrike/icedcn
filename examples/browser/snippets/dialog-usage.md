## Usage

Build the trigger in the first closure and the panel in the second.

```rust
use icedcn::widget::{button, dialog};

dialog(
    |d| d.trigger(button("Open")),
    |_| dialog::header(
        "Are you absolutely sure?",
        "This action cannot be undone. This will permanently delete your account and remove your data from our servers.",
    ),
)
```
