```rust
use crate::icon;
use icedcn::widget::{button, code_block};

#[derive(Debug, Clone)]
enum Message {
    Wrap,
    Download,
}

code_block![
    code_block::header![
        code_block::title![terminal, "Terminal"],
        code_block::actions![
            button(icon::wrap_text())
                .style(button::Ghost)
                .square()
                .pressed(wrap)
                .on_press(Message::Wrap),
            button(icon::download())
                .style(button::Ghost)
                .square()
                .on_press(Message::Download),
            code_block::copy_button(OUTPUT),
        ],
    ],
    code_block::content(OUTPUT).wrap(wrap),
]

const OUTPUT: &str = r##"$ cargo clippy -- -D warnings
    Checking notebook v0.1.0
error: writing `&String` instead of `&str` involves a new object where a slice will do
 --> src/search.rs:4:18
  |
4 | fn matches(query: &String, title: &str) -> bool {
  |                   ^^^^^^^ help: change this to: `&str`"##;
```
