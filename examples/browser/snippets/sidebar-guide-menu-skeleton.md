## SidebarMenuSkeleton

Show placeholder rows while destinations load.

```rust
use icedcn::View;
use icedcn::widget::sidebar;

pub fn loading<'a>() -> View<'a> {
    sidebar::menu((0..5).map(|_| sidebar::menu_skeleton().into())).into()
}
```
