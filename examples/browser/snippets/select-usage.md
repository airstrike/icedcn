## Usage

```rust
use icedcn::widget::select;

select([
    select::item("light", "Light"),
    select::item("dark", "Dark"),
    select::item("system", "System"),
])
.placeholder("Theme")
.width(180)
```
