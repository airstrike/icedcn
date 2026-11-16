```rust
use iced::Fill;
use icedcn::widget::{code_block, column};

column![
    code_block(OUTPUT),
    code_block(code_block::content(OUTPUT).wrap(true)),
]
.width(Fill)
.spacing(16)

const OUTPUT: &str = r##"error: failed to read notebook index from /home/reader/.local/share/notebook/collections/research/field-notes/archive/index.toml: No such file or directory (os error 2)"##;
```
