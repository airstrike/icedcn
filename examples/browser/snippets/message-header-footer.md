```rust
frame(
    column![
        message(bubble("I already checked the logs.").style(Muted)).header("Olivia"),
        message(bubble(
            "Send the report to the team. Ping @alex if you need help."
        ))
        .align(Right)
        .footer(
            row![
                text("Read").size(12.).medium(),
                text("Yesterday").size(12.).weight(Weight::Normal)
            ]
            .spacing(4)
        )
    ]
    .spacing(32),
)
```
