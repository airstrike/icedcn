## Provider handle

The provider closure receives a `sidebar::Handle`. Read `is_open()` for the current open state, use `width()` for the panel’s current width and wrap the panel in `sidebar(...)` to apply its visibility and geometry.

```rust
sidebar::provider(|s| {
    row![
        s.sidebar(sidebar::fixed(navigation())),
        sidebar::main(column![sidebar::trigger(), workspace()]),
    ].into()
})
```
