```rust
scroller(rows)
    .default_scroll_position(Position::Start)
    .on_reader_change(Event::Reader)

// The footer uses reader.can_scroll_up and reader.can_scroll_down.
```
