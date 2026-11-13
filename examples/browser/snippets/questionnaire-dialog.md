```rust
let questions = vec![
    item("scope", "Which files are in scope?")
        .required(true)
        .description("Choose how broadly the agent can update the workspace.")
        .choices([
            choice("component", "Component only"),
            choice("feature", "Complete feature directory"),
            choice("workspace", "Any related workspace file"),
        ]),
    item("tests", "How much verification is needed?")
        .required(true)
        .description("Choose the checks the agent should run before handoff.")
        .choices([
            choice("targeted", "Targeted tests"),
            choice("package", "Package tests"),
            choice("full", "Full workspace verification"),
        ]),
];
questionnaire(questions)
    .submit_label("Send answer")
    .on_submit(Event::Submit)
```
