```rust
frame(
    column![
        bubble("Did you remove the stale route?").style(Secondary),
        bubble("Yes, removed it from the registry.")
            .align(Right)
            .reactions(
                tooltip(
                    button(icon::check())
                        .size(Size::Xs)
                        .square()
                        .style(button::Ghost),
                    "Read on Jan 5, 2026 at 4:32 PM"
                ),
                Right,
                Bottom
            )
    ]
    .spacing(16),
)
```
