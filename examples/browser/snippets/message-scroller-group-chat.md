```rust
let joined = status_marker("Rocky has joined the chat")
    .style(status_marker::Separator);
let reply = message(bubble(rockys_reply).style(bubble::Tinted))
    .header("Rocky");
scroller(history.chain([
    scroller::item(3, joined).anchor(true),
    scroller::item(4, reply),
]))
```
