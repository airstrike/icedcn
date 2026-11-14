```rust
use icedcn::widget::combobox;

const FRAMEWORKS: [&str; 5] = ["iced", "egui", "Slint", "GTK", "Druid"];

combobox(FRAMEWORKS)
    .placeholder("Select a framework")
    .icon(icon::chevron_down)
    .invalid(true)
```
