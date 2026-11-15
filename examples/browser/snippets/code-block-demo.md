```rust
use crate::icon;
use icedcn::widget::{CodeBlock, code_block, row};

#[derive(Clone, Debug)]
pub enum Message {
    Wrap,
    Save,
}

pub fn view<'a>(source: &'a str, wrap: bool) -> CodeBlock<'a, Message> {
    let mut block = code_block(source).language("rust").start_line(1);
    for line in 6..=10 {
        block = block.line_style(line, code_block::LineStyle::Highlighted);
    }
    block
        .font("Geist Mono")
        .wrap(wrap)
        .title(row![icon::file_code().size(14), code_block::title("src/view.rs")].spacing(6))
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
