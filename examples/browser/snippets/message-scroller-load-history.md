```rust
let rows = all_messages.iter().enumerate().skip(first_visible).map(|(i, turn)| {
    scroller::item(i as u64, message(bubble(turn.text)))
});
scroller(rows).preserve_scroll_on_prepend(true)
```
