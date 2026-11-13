```rust
let css = r##"@import "tailwindcss";

:root {
  --radius: 0.625rem;
}"##;
let python = r##"def greet(name: str) -> str:
    return f"Hello, {name}!""##;
let plain = r##"A language that is not in the list renders as plain text."##;

column![
    code_block(css).title("app/globals.css"),
    code_block(python).title("main.py"),
    code_block(plain).title("notes.txt"),
].spacing(12)
```
