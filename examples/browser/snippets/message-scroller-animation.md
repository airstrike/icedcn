```rust
// Animate the content of a newly appended item. Its logical row remains in place.
let content = entrance(bubble(text::small(turn.text)).into(), preset, frame);
message_scroller::item(id, message(content)).anchor(true)
```
