```rust
use icedxdy::widget::combobox;

const FRAMEWORKS: [&str; 5] = ["iced", "egui", "Slint", "GTK", "Druid"];

combobox(FRAMEWORKS)
    .multiple(true)
    .auto_highlight(true)
    .default_values(["iced"])
    .width(320)
```
