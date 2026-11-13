```rust
let rows = messages.iter().map(|turn| {
    message_scroller::item(turn.id, message(bubble(turn.text)))
        .anchor(turn.role == Role::User)
});
message_scroller(rows).on_reader_change(Event::Reader)
```
