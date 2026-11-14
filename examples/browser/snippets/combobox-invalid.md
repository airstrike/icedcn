```rust
use icedcn::widget::combobox;

const FRAMEWORKS: [&str; 5] = ["iced", "egui", "Slint", "GTK", "Druid"];

combobox(FRAMEWORKS)
    .placeholder("Select a framework")
    .invalid(true)
```
