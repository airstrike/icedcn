## SidebarMenuSkeleton

Show placeholder rows while destinations load.

```rust
use icedcn::Slot;
use icedcn::widget::sidebar;

pub fn loading<'a>() -> Slot<'a, ()> {
    sidebar::menu((0..5).map(|_| sidebar::menu_skeleton().into())).into()
}
```
