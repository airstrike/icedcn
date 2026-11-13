```rust
frame(
    column![
        bubble("This bubble is aligned to the start. This is the default alignment.")
            .style(Muted),
        bubble("This bubble is aligned to the end. Use this for user messages.").align(Right)
    ]
    .spacing(32),
)
```
