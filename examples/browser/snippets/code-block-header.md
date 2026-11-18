```rust
use crate::icon;
use icedcn::widget::code_block;

code_block(OUTPUT).header(
    code_block::header(code_block::title("Terminal").icon(icon::terminal()))
        .actions(code_block::copy_button(COMMAND)),
)

const COMMAND: &str = "cargo check -p icedcn_widget";

const OUTPUT: &str = r##"$ cargo check -p icedcn_widget
    Checking icedcn_core v0.1.0
    Checking icedcn_widget v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.80s"##;
```
