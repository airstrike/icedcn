# Composition

Use `navigation_menu::menu` for expandable groups and `navigation_menu::link` for standalone links.

```rust
navigation_menu([
    navigation_menu::menu("Products", || {
        menu::group([menu::item("Browse products")
            .on_select(Message::Products)
            .into()])
    }),
    navigation_menu::link("Docs", Message::Docs),
])
```
