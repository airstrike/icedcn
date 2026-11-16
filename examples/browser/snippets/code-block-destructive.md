```rust
use icedcn::Destructive;
use icedcn::widget::code_block;

code_block![
    code_block::header![
        "Error",
        code_block::actions![code_block::copy_button(ERROR)],
    ],
    code_block::content(ERROR).wrap(true),
]
.style(Destructive)

const ERROR: &str = r##"thread 'main' panicked at src/notebook.rs:18:22:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` to display a backtrace"##;
```
