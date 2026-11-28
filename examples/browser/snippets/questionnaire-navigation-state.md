```rust
use icedcn::widget::questionnaire;
use icedcn::widget::questionnaire::{choice, item};

let questions = vec![
    item("permission", "What may the agent modify?")
        .required(true)
        .description("Next is intentionally disabled until an answer is selected.")
        .choices([
            choice("files", "Project files"),
            choice("tests", "Project files and tests"),
            choice("config", "Files, tests, and configuration"),
        ]),
    item("verification", "What must pass before completion?")
        .required(true)
        .choices([
            choice("tests", "Tests"),
            choice("types", "Tests and types"),
            choice("all", "Tests, types, and visual QA"),
        ]),
];
questionnaire(questions)
    .block_unanswered(true)
    .submit_label("Save permissions")
    .on_submit(Event::Submit)
```
