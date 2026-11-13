```rust
field::group([
    field(switch())
        .orientation(Horizontal)
        .control_last()
        .label("Share across devices")
        .description("Focus is shared across devices, and turns off when you leave the app.")
        .card()
        .into(),
    field(switch().default_checked(true))
        .orientation(Horizontal)
        .control_last()
        .label("Enable notifications")
        .description("Receive notifications when focus mode is enabled or disabled.")
        .card()
        .into(),
])
.width(384)
```
