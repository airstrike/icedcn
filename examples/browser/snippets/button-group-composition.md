## Composition

Place buttons, inputs and separators inside `button_group!` in their visual order.

```rust
use icedcn::widget::{button, button_group};
use icedcn::widget::button::group;

button_group![button("Copy"), group::separator(), button("Paste")]
```
