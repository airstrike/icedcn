```rust
use icedcn::widget::questionnaire;
use icedcn::widget::questionnaire::{Shortcuts, choice, item};

let questions = vec![
    item("context", "What context should the agent inspect?")
        .required(true)
        .multiple(true)
        .description("Select every source that may affect the implementation.")
        .choices([
            choice("source", "Relevant source files"),
            choice("tests", "Existing tests"),
            choice("docs", "Architecture documentation"),
            choice("history", "Recent commit history"),
        ]),
];
questionnaire(questions)
    .shortcuts(Shortcuts::Letters)
    .submit_label("Share context")
    .on_submit(Event::Submit)
```
