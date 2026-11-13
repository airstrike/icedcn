```rust
let request = Command {
    sequence: next_request,
    target: Target::Item(id),
};
message_scroller(rows).smooth_commands(true).command(Some(request))
```
