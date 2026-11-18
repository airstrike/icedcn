```rust
use crate::icon;
use icedcn::widget::code_block;
use icedcn::widget::code_block::ansi;

code_block(code_block::content(OUTPUT).language("ansi").wrap(true)).header(
    code_block::header(code_block::title("Terminal").icon(icon::terminal()))
        .actions(code_block::copy_button(ansi::strip(OUTPUT))),
)

const OUTPUT: &str = concat!(
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
