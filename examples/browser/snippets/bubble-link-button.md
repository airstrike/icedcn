```rust
frame(
    column![
        bubble("How can I help you today?").style(Muted),
        bubble::group([
            bubble(suggestion(
                "I forgot my password",
                "You clicked forgot password"
            ))
            .style(Tinted)
            .align(Right)
            .into(),
            bubble(suggestion(
                "I need help with my subscription",
                "You clicked help with subscription"
            ))
            .style(Tinted)
            .align(Right)
            .into(),
            bubble(suggestion(
                "Something else. Talk to a human.",
                "You clicked something else. Talk to a human."
            ))
            .style(Tinted)
            .align(Right)
            .into()
        ])
    ]
    .spacing(32),
)
```
