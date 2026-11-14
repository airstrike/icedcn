```rust
let rows = turns.iter().enumerate().map(|(index, turn)| {
    let content = message(bubble(turn.text));
    scroller::item(index as u64, content)
        .anchor(turn.role == selected_role)
});
scroller(rows)
```
