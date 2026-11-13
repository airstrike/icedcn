```rust
frame(
    column![
        bubble("This is the default primary bubble."),
        bubble("This is the secondary variant.")
            .style(Secondary)
            .align(Right),
        bubble("This one is muted. It uses a lower emphasis color for the chat bubble.")
            .style(Muted)
            .reactions("👍", Right, Bottom),
        bubble(
            "This one is tinted. The tint is a softer color derived from the primary color."
        )
        .style(Tinted)
        .align(Right),
        bubble("We can also use an outlined variant.").style(Outline),
        bubble("Or a destructive variant with a reaction.")
            .style(Destructive)
            .align(Right)
            .reactions("🔥", Right, Bottom),
        bubble(ghost_text()).style(Ghost)
    ]
    .spacing(48),
)
```
