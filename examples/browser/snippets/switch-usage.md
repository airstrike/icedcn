## Usage

A switch keeps its checked state across view rebuilds. Set an initial value with `.default_checked(true)`. To share the value with another part of the application, pass the current state and its change handler.

```rust
use icedcn::Slot;
use icedcn::widget::switch;

fn notifications<M: 'static>(
    enabled: bool,
    changed: impl Fn(bool) -> M + 'static,
) -> Slot<'static, M> {
    switch().checked(enabled).on_checked_change(changed).into()
}
```
