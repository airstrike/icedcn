```rust
attachment(name)
    .vertical(true)
    .media(picture(index, 104))
    .description(meta)
    .trigger(move |content| preview_button(content, Some(Edit::Open(url))))
    .action(remove(name))
```
