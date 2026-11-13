```rust
let source = r##"Error: The model "anthropic/claude-sonnet-4.5" returned an error: Request too large. The prompt is 212,431 tokens and the context window is 200,000 tokens. Remove some messages or summarize the conversation and try again."##;

column![code_block(source), code_block(source).wrap(true)].spacing(16)
```
