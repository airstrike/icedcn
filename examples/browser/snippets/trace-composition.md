# Composition

```rust
trace([
    trace::line("Read package.json")
        .icon(icon::file_text().size(16))
        .into(),
    trace::disclosure(
        || trace::line("Ran pnpm --version"),
        || code_block("10.33.4").into(),
    )
    .into(),
])
```
