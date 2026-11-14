```rust
use icedxdy::widget::combobox;

const FRAMEWORKS: [&str; 5] = ["iced", "egui", "Slint", "GTK", "Druid"];

combobox(FRAMEWORKS)
    .placeholder("Select a framework")
    .icon(icon::chevron_down)
    .clearable(true)
    .default_value("iced")
```
