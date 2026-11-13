```rust
CodeBlock::lines([
    (LineStyle::Default, rich_text![span("✓").color(Token::Success), " src/format.test.ts"].into()),
    (LineStyle::Default, rich_text![span("✗").color(Token::Destructive), " src/stream.test.ts"].into()),
]).title("Terminal").wrap(true)
```
