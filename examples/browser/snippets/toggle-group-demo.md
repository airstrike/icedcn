```rust
use crate::icon;
use icedcn::widget::toggle_group;

toggle_group(|| {
    [
        toggle_group::item("bold", icon::bold()),
        toggle_group::item("italic", icon::italic()),
        toggle_group::item("underline", icon::underline()),
    ]
})
.mode(toggle_group::Mode::Multiple)
.style(toggle_group::Outline)
```
