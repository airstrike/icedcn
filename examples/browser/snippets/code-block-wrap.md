```rust
use iced::Font;
use icedxdy::{
    Slot,
    widget::{code_block, column},
};
pub fn view<Message: Clone + 'static>() -> Slot<'static, Message> {
    let font = Font::new("Geist Mono");
    column![
        code_block(WRAP).font(font),
        code_block(WRAP).font(font).wrap(true)
    ]
    .spacing(16)
    .into()
}

const WRAP: &str = r##"error: failed to read notebook index from /home/reader/.local/share/notebook/collections/research/field-notes/archive/index.toml: No such file or directory (os error 2)"##;
```
