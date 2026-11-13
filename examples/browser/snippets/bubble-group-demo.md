```rust
frame(
    column![
        bubble("Can you tell me what's the issue?").style(Muted),
        bubble::group([
            bubble("You tell me!").align(Right).into(),
            bubble("It worked yesterday. You broke it!")
                .align(Right)
                .into(),
            bubble("Find the bug and fix it.")
                .align(Right)
                .reactions("👀", Left, Bottom)
                .into()
        ]),
        bubble("Want me to diff yesterday's you against today's you? It's a bit embarrassing.")
            .style(Muted)
    ]
    .spacing(32),
)
```
