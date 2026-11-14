```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message>() -> CodeBlock<'a, Message> {
    code_block(FLOATING_ACTIONS)
        .font("Geist Mono")
        .copyable(true)
}

const FLOATING_ACTIONS: &str = r##"$ cargo add serde --features derive
    Updating crates.io index
      Adding serde v1.0 to dependencies
             Features:
             + derive
             + std"##;
```
