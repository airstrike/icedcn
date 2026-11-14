```rust
frame(message::group([
    message(bubble("I checked the registry addresses.").style(Muted))
        .avatar(space().width(32).height(32))
        .into(),
    message(
        bubble("The component and example JSON now live under the UI registry.").style(Muted),
    )
    .avatar(portrait(0, "AC"))
    .into(),
]))
```
