```rust
let rows = all_messages.iter().enumerate().skip(first_visible).map(|(i, turn)| {
    message_scroller::item(i as u64, message(bubble(turn.text)))
});
message_scroller(rows).preserve_scroll_on_prepend(true)
```
