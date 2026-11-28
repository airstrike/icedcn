```rust
use icedcn::widget::questionnaire;
use icedcn::widget::questionnaire::{choice, item};

let questions = vec![
    item("task", "What kind of change is this?")
        .required(true)
        .description("Choose the category that best describes the work.")
        .choices([
            choice("feature", "New feature"),
            choice("fix", "Bug fix"),
            choice("refactor", "Refactor"),
        ]),
    item("constraints", "Are there any implementation constraints?")
        .description("Answer if needed, or intentionally skip this question.")
        .choices([
            choice("no-dependencies", "Do not add dependencies"),
            choice("no-migrations", "Do not change the database"),
            choice("preserve-api", "Preserve the public API"),
        ])
        .input(
            "Another implementation constraint",
            "Describe another constraint…",
        ),
    item("review", "How should the work be reviewed?")
        .required(true)
        .description("Choose the checks the agent should complete before handoff.")
        .choices([
            choice("tests", "Run the test suite"),
            choice("diff", "Review the final diff"),
            choice("both", "Tests and diff review"),
        ]),
];
questionnaire(questions)
    .allow_skip(true)
    .submit_label("Submit brief")
    .on_submit(Event::Submit)
```
