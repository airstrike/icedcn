# Composition

```rust
trace([
    trace::line("Read Cargo.toml")
        .icon(icon::file_text().size(16))
        .into(),
    trace::disclosure(
        || trace::line("Ran cargo --version"),
        || code_block("cargo 1.89.0").into(),
    )
    .into(),
])
```
