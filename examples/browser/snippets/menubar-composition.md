# Composition

Group commands under named triggers. Use `menu::item`, `menu::separator`, and `menu::sub` inside each panel.

```rust
use icedcn::widget::{menu, menubar};

menubar([menubar::menu("File", || {
    menu::group([menu::item("New File").into()])
})])
```
