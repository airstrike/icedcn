```rust
empty::empty().media(empty::media(avatar::group([
    avatar(Some(alex_handle())).fallback("AC").size(48.0), avatar(Some(maxleiter_handle())).fallback("LR").size(48.0), avatar(Some(evilrabbit_handle())).fallback("ER").size(48.0)])))
    .title("No Team Members").description("Invite your team to collaborate on this project.")
    .content(button![icon::plus(), "Invite Members"].size(Size::Sm))
```
