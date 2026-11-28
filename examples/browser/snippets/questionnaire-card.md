```rust
use icedcn::widget::questionnaire;
use icedcn::widget::questionnaire::{choice, item};

let questions = vec![
    item("task", "What should the agent work on?")
        .required(true)
        .description("Choose the task that should be handled next.")
        .choices([
            choice("fix", "Fix the failing tests"),
            choice("refactor", "Refactor the data layer"),
            choice("docs", "Update the integration guide"),
        ]),
    item("output", "What should the final handoff include?")
        .required(true)
        .description("Pick the level of detail needed for review.")
        .choices([
            choice("summary", "Summary only"),
            choice("files", "Summary and changed files"),
            choice("review", "Full review handoff"),
        ]),
];
questionnaire(questions)
    .submit_label("Create task")
    .on_submit(Event::Submit)
```
