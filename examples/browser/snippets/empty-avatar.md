```rust
empty::empty().media(empty::media(avatar(Some(shadcn_handle())).fallback("LR").size(48.0)))
    .title("User Offline").description("This user is currently offline. You can leave a message to notify them or try again later.")
    .content(button("Leave Message").size(Size::Sm))
```
