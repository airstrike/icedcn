```rust
use crate::icon;
use iced::Fill;
use icedcn::Size;
use icedcn::widget::{code_block, column};

column![
    code_block![
        code_block::header![
            code_block::title![terminal, "Terminal"],
            code_block::actions![code_block::copy_button(OUTPUT)],
        ],
        code_block::content(OUTPUT).wrap(true),
    ],
    code_block![
        code_block::header![
            code_block::title![terminal, "Terminal"],
            code_block::actions![code_block::copy_button(OUTPUT)],
        ],
        code_block::content(OUTPUT).wrap(true),
    ]
    .size(Size::Sm),
]
.width(Fill)
.spacing(16)

const OUTPUT: &str = r##"$ cargo check -p notebook
    Checking notebook v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s"##;
```
