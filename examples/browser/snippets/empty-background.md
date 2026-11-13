```rust
container(empty::empty().media(empty::media(icon::bell()).style(empty::Icon)).title("No Notifications")
    .description("You're all caught up. New notifications will appear here.")
    .content(button![icon::refresh_ccw(), "Refresh"].style(button::Outline))).width(Fill)
    .style(|theme| Style { background: Some(theme.muted.base.alpha(0.3).into()), ..Style::default() })
```
