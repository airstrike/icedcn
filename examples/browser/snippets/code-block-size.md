```rust
use icedcn::{
    Slot,
    widget::{code_block, column},
};
pub fn view<'a, Message: 'a>() -> Slot<'a, Message> {
    let block = || {
        code_block(SIZE)
            .font("Geist Mono")
            .wrap(true)
            .title(code_block::terminal_title("Terminal"))
            .actions(code_block::copy_button(SIZE))
    };
    column![block(), block().compact(true)].spacing(16).into()
}

const SIZE: &str = r##"$ cargo check -p notebook
    Checking notebook v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s"##;
```
