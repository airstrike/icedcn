```rust
frame(
    column![
        bubble("How can I help you today?").style(Muted),
        bubble(review()).style(Muted).align(Right)
    ]
    .spacing(32),
)
```
