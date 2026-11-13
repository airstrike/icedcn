```rust
attachment::group(files.into_iter().map(|(name, metadata, media)| {
    attachment(name).media(media).width(256).description(metadata).action(remove(name)).into()
}))
```
