```rust
let source = r##"TypeError: Cannot read properties of undefined (reading 'map')
    at MessageList (components/message-list.tsx:18:22)
    at renderWithHooks (react-dom.development.js:16305:18)"##;

code_block(source).title("Error").wrap(true)
    .style(theme::container::CodeBlock + Bg(Token::Destructive) + Alpha(0.10) + Fg(Token::Destructive))
```
