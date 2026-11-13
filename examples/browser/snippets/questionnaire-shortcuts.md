```rust
let questions = vec![
    item("action", "What should the agent do next?")
        .required(true)
        .description("Use the displayed shortcut or navigate with the keyboard.")
        .choices([
            choice("inspect", "Inspect the implementation"),
            choice("tests", "Run the relevant tests"),
            choice("patch", "Prepare the patch"),
        ]),
];
questionnaire(questions)
    .shortcuts(shortcuts)
    .submit_label("Confirm action")
    .on_submit(Event::Submit)
```
