```rust
let rows = messages.iter().map(|turn| {
    scroller::item(turn.id, message(bubble(turn.text)))
        .anchor(turn.role == Role::User)
});
scroller(rows).on_reader_change(Event::Reader)
```
