```rust
use icedcn::widget::questionnaire;
use icedcn::widget::questionnaire::{choice, item};

let questions = vec![
    item("scope", "What may the agent change?")
        .required(true)
        .description("The host stores the active checkpoint while Questionnaire navigates.")
        .choices([
            choice("component", "Only the target component"),
            choice("tests", "Component and related tests"),
            choice("feature", "The complete feature area"),
        ]),
    item("checks", "Which verification level should it use?")
        .required(true)
        .choices([
            choice("targeted", "Targeted tests"),
            choice("package", "Package tests and typecheck"),
            choice("full", "Full workspace verification"),
        ]),
    item("output", "What should the agent return when finished?")
        .required(true)
        .choices([
            choice("summary", "Concise summary"),
            choice("diff", "Summary with changed files"),
            choice("handoff", "Detailed implementation handoff"),
        ]),
];
questionnaire(questions)
    .item(active)
    .on_item_change(Event::Navigate)
    .submit_label("Save workflow")
    .on_submit(Event::Submit)
```
