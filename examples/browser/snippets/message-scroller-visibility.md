```rust
scroller(rows).on_reader_change(Event::Reader)
// reader.current_anchor identifies the latest anchor above the viewport.
// reader.visible contains the stable keys of every visible row.
```
