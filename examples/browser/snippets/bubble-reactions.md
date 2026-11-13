```rust
frame(
    column![
        bubble("I don't need tests, I know my code works.")
            .style(Muted)
            .align(Right)
            .reactions("👍 😮", Left, Bottom),
        bubble("Bold. Fine I'll add some tests. I'll let you know when they're done.")
            .style(Muted)
            .reactions("👀 🚀 +2", Right, Bottom),
        bubble("Tests passed on the first try. All 142 of them. Looking good!")
            .align(Right)
            .reactions("🎉 👏", Left, Top),
        bubble("Are you sure I can run this command?")
            .style(Destructive)
            .reactions(
                toast::trigger(
                    button("Yes, run it").size(Size::Xs).style(button::Ghost),
                    toast::toast("You clicked yes, running command...")
                        .kind(toast::Kind::Success)
                ),
                Right,
                Bottom
            )
    ]
    .spacing(48),
)
```
