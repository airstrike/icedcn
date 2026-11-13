```rust
let joined = status_marker("Rocky has joined the chat")
    .style(status_marker::Separator);
let reply = message(bubble(rockys_reply).style(bubble::Tinted))
    .header("Rocky");
message_scroller(history.chain([
    message_scroller::item(3, joined).anchor(true),
    message_scroller::item(4, reply),
]))
```
