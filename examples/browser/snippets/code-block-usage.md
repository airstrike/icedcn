## Usage

```rust
use icedcn::widget::code_block;
```

```rust
use icedcn::widget::code_block;

code_block(output).header(
    code_block::header(code_block::title("Terminal")).actions(code_block::copy_button(output)),
)
```
