```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn terminal<'a, Message>() -> CodeBlock<'a, Message> {
    code_block::ansi(TRANSCRIPT)
        .font("Geist Mono")
        .title(code_block::terminal_title("Terminal"))
        .copyable(true)
}

const TRANSCRIPT: &str = concat!(
    "$ cargo test --color always\n",
    "    \x1b[2mRunning tests/search.rs\x1b[22m\n",
    "running 3 tests\n",
    "test blank_query ... \x1b[32mok\x1b[39m\n",
    "test exact_match ... \x1b[32mok\x1b[39m\n",
    "test mixed_case ... \x1b[31mFAILED\x1b[39m\n",
    "\n",
    "test result: \x1b[31mFAILED\x1b[39m. 2 passed; 1 failed; 0 ignored",
);
```
