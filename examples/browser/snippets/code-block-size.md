```rust
let source = r##"$ pnpm add ai @ai-sdk/react zod
Progress: resolved 412, reused 398, downloaded 14, added 14, done
Done in 2.8s using pnpm v10.18.0"##;

column![
    code_block(source).title("Terminal"),
    code_block(source).title("Terminal").compact(true),
].spacing(16)
```
