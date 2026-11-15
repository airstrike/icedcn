```rust
use crate::icon;
use icedcn::widget::{CodeBlock, code_block, row};

#[derive(Clone, Debug)]
pub enum Message {
    Wrap,
    Save,
}

pub fn view<'a>(output: &'a str, wrap: bool) -> CodeBlock<'a, Message> {
    code_block(output)
        .font("Geist Mono")
        .wrap(wrap)
        .title(code_block::terminal_title("Terminal"))
        .actions(
            row![
                code_block::action(icon::wrap_text().size(14), "Wrap lines")
                    .active(wrap)
                    .on_press(Message::Wrap),
                code_block::action(icon::download().size(14), "Download").on_press(Message::Save),
            ]
            .spacing(4),
        )
        .copyable(true)
}
```
