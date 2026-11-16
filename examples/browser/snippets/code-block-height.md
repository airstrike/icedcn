```rust
use icedcn::widget::code_block;

let logs = (0..40)
    .map(|i| format!("[{:02}] Indexed notebook in {}ms", i + 1, 12 + (i * 7) % 40))
    .collect::<Vec<_>>()
    .join("\n");

code_block![
    code_block::header![
        "Logs",
        code_block::actions![code_block::copy_button(&logs)],
    ],
    code_block::content(&logs).max_height(192),
]
```
