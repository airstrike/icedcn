```rust
frame(
    column![
        bubble("Hey there! what's up?").align(Right),
        bubble::group([
            bubble("Hey! Want to see chat bubbles?").style(Muted).into(),
            bubble(
                "I can group messages, switch sides, and keep the whole thread easy to scan."
            )
            .style(Muted)
            .reactions("👍", Right, Bottom)
            .into()
        ]),
        bubble("Sure. Hit me with your best demo.").align(Right),
        bubble("Yes. You are reading a demo that is demoing itself. Very meta. Very on-brand.")
            .style(Muted)
            .reactions("👍 🔥 👀 +2", Right, Bottom)
    ]
    .spacing(32),
)
```
