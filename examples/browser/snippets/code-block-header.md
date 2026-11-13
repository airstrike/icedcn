```rust
let source = r##"$ pnpm add ai @ai-sdk/react zod
Packages: +14
Progress: resolved 412, reused 398, downloaded 14, added 14, done
Done in 2.8s using pnpm v10.18.0"##;

code_block(source)
    .title("Terminal")
    .actions(button(icon::copy()).on_press(Message::CopyCommand))
```
