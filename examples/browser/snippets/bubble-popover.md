```rust
frame(
    column![
        bubble("Run the build script.").align(Right),
        bubble("Failed to run the command.")
            .style(Destructive)
            .reactions(
                popover(
                    |p| p.trigger(
                        button(icon::info())
                            .size(Size::Xs)
                            .square()
                            .style(button::Ghost)
                    ),
                    |_| popover::header(
                        "Command failed with exit code 1",
                        "ENOENT: no such file or directory, open pnpm-lock.yaml"
                    )
                ),
                Right,
                Bottom
            )
    ]
    .spacing(16),
)
```
