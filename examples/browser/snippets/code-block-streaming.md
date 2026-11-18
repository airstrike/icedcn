```rust
use icedcn::widget::code_block;

// `received` grows as the stream arrives; `complete` is set when it ends.
code_block(
    code_block::content(&received)
        .language("rust")
        .line_numbers(true),
)
.header(
    code_block::header("src/matcher.rs")
        .actions(code_block::copy_button(&received).disabled(!complete)),
)
```
