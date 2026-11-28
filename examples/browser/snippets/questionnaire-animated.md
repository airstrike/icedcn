```rust
use icedcn::widget::questionnaire;
use icedcn::widget::questionnaire::{choice, item};

let questions = vec![
    item("task", "What should the agent do?")
        .required(true)
        .description("Choose the task for this run.")
        .choices([
            choice("implement", "Implement the requested change"),
            choice("debug", "Debug the current behavior"),
            choice("review", "Review the implementation"),
        ]),
    item("review", "How should the work be reviewed?")
        .required(true)
        .description("Select the verification depth.")
        .choices([
            choice("targeted", "Targeted checks"),
            choice("complete", "Complete test suite"),
            choice("manual", "Tests and manual QA"),
        ]),
    item("delivery", "How should the result be delivered?")
        .required(true)
        .description("Choose the final handoff format.")
        .choices([
            choice("summary", "Concise summary"),
            choice("diff", "Summary and changed files"),
            choice("handoff", "Detailed review handoff"),
        ]),
];
questionnaire(questions)
    .animated(true)
    .submit_label("Save workflow")
    .on_submit(Event::Submit)
```
