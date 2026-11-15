## Usage

```rust
use icedcn::widget::accordion;

accordion([
    accordion::item("item-1", "Is it accessible?")
        .content("Use Tab to focus a header, then press Enter or Space to toggle it."),
])
.default_value(["item-1"])
```
