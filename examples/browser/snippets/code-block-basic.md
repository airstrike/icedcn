```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<'a, Message>() -> CodeBlock<'a, Message> {
    code_block(BASIC).font("Geist Mono")
}

const BASIC: &str = r##"$ cargo build --release
   Compiling icedcn_core v0.1.0
   Compiling icedcn_widget v0.1.0
   Compiling icedcn v0.1.0
   Compiling notebook v0.1.0
    Finished `release` profile [optimized] target(s) in 4.20s"##;
```
