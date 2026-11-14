```rust
use icedcn::{
    Slot,
    widget::{code_block, column},
};
pub fn view<'a, Message: 'a>() -> Slot<'a, Message> {
    column![
        code_block(WRAP).font("Geist Mono"),
        code_block(WRAP).font("Geist Mono").wrap(true)
    ]
    .spacing(16)
    .into()
}

const WRAP: &str = r##"error: failed to read notebook index from /home/reader/.local/share/notebook/collections/research/field-notes/archive/index.toml: No such file or directory (os error 2)"##;
```
