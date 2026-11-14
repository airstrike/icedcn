## Custom Items

Use `combobox::item(value, label)` when an option's stable value differs from its displayed label. Selection publishes the stable value. Use `.format_item(...)` for arbitrary composed content inside each option.

```rust
use icedcn::widget::{combobox, text};

combobox::options([
    combobox::item("iced", "iced"),
    combobox::item("egui", "egui"),
    combobox::item("slint", "Slint"),
])
.placeholder("Select a framework")
.format_item(|option| text::small(option.label.clone()))
```
