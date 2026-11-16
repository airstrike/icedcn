```rust
use icedcn::widget::code_block;

code_block![
    code_block::actions![code_block::copy_button(OUTPUT)],
    OUTPUT,
]

const OUTPUT: &str = r##"$ cargo add serde --features derive
    Updating crates.io index
      Adding serde v1.0 to dependencies
             Features:
             + derive
             + std"##;
```
