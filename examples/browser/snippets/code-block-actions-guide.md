```rust
use crate::icon;
use icedcn::widget::{button, code_block};

code_block(code_block::content(output).wrap(wrap)).header(
    code_block::header("Terminal").actions(code_block::actions![
        button(icon::wrap_text())
            .style(button::Ghost)
            .square()
            .pressed(wrap)
            .on_press(Message::Wrap),
        code_block::copy_button(output),
    ]),
)
```

Buttons in actions are extra small. A ghost button in the header is muted at rest and turns to the foreground color on hover. `.pressed(wrap)` keeps a wrap toggle in the foreground color while it is on.
