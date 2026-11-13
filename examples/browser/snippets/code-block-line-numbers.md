```rust
let source = r##"{
  "name": "chat",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build"
  }
}"##;

code_block(source).start_line(1)
```
