```rust
column![
    photos(state, false),
    attachment("sales-dashboard.pdf").media(spinner()).description("Uploading · 64%").state(State::Uploading).action(remove("sales-dashboard.pdf")),
    attachment("message-renderer.tsx").media(icon::file_code()).description("TypeScript · 12 KB").action(remove("message-renderer.tsx")),
].spacing(12)
```
