```rust
let source = r##"$ node --version
v24.18.0"##;

collapsible(|c| column![
    c.trigger(button("Ran node --version")),
    c.content(code_block(source).compact(true).wrap(true)),
].into())
```
