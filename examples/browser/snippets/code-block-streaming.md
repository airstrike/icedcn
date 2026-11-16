```rust
use icedcn::widget::code_block;

// `received` grows as the stream arrives; `complete` is set when it ends.
code_block![
    code_block::header![
        "src/matcher.rs",
        code_block::actions![code_block::copy_button(&received).disabled(!complete)],
    ],
    code_block::content(&received)
        .language("rust")
        .line_numbers(true),
]
```
