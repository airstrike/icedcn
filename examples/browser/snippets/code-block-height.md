```rust
let source = (0..40).map(|i| format!("[{:02}] GET /api/chat 200 in {}ms", i + 1, 12 + (i * 7) % 40)).collect::<Vec<_>>().join("\n");

code_block(source).title("Logs").height(160.0)
```
