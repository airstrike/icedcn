```rust
use icedxdy::widget::{trace, text};

trace::disclosure(||trace::line("Worked for 2m 4s, 1 command failed").status(Status::Error),move||column![
    trace([
        trace::disclosure(||line("Thought",icon::brain).description("12s"),||text("I should check which package managers are installed before I scaffold anything.").into()).into(),
        trace::disclosure(||line("Ran 2 commands, 1 failed",icon::terminal).status(Status::Error),||trace([
            line("Ran node --version",icon::terminal).into(),result("Failed to run pnpm --version",icon::terminal,"command not found: pnpm",Status::Error,false)
        ]).into()).default_open(true).into()
    ]),
    text("Node is installed and pnpm is not. Reading the docs config."),
    local("Read apps/v4/lib/docs.ts","turn",index)
].spacing(8).into()).default_open(true).into()
```
