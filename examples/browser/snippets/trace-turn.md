```rust
use icedcn::widget::{trace, text};

trace::disclosure(||trace::line("Worked for 2m 4s, 1 command failed").status(Status::Error),move||column![
    trace([
        trace::disclosure(||line("Thought",icon::brain).description("12s"),||text("I should check which Rust tools are available before creating a project.").into()).into(),
        trace::disclosure(||line("Ran 2 commands, 1 failed",icon::terminal).status(Status::Error),||trace([
            line("Ran rustc --version",icon::terminal).into(),result("Failed to run cargo --version",icon::terminal,"command not found: cargo",Status::Error,false)
        ]).into()).default_open(true).into()
    ]),
    text("Found rustc. Cargo is missing from PATH. Reading the project documentation."),
    local("Read examples/browser/src/page.rs","turn",index)
].spacing(8).into()).default_open(true).into()
```
