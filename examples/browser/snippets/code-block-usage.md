## Usage

```rust
use icedcn::widget::code_block;
```

```rust
code_block![
    code_block::header![
        code_block::title("Terminal"),
        code_block::actions![code_block::copy_button(output)],
    ],
    output,
]
```
