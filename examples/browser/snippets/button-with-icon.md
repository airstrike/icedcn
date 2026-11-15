```rust
use crate::icon;
use icedcn::Size;
use icedcn::widget::button;

let branch = icon::git_branch().size(14).line_height(1.0);

button(branch)
    .push("New Branch")
    .size(Size::Sm)
    .style(button::Outline)
```
