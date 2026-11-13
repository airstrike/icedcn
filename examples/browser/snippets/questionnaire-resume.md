```rust
let questions = vec![
    item("change", "What kind of migration is this?")
        .required(true)
        .description("This answer was saved during the previous session.")
        .choices([
            choice("incremental", "Incremental migration"),
            choice("cutover", "Single cutover"),
        ]),
    item("verification", "How should the migration be verified?")
        .required(true)
        .multiple(true)
        .description("These checks were selected during the previous session.")
        .choices([
            choice("tests", "Run migration tests"),
            choice("typecheck", "Run the typecheck"),
            choice("manual", "Perform a manual smoke test"),
        ]),
    item("notes", "Anything else the agent should remember?")
        .description("This note was saved with the draft.")
        .input("Saved migration note", ""),
];
questionnaire(questions)
    .default_item("verification")
    .default_answers(saved)
    .reset_action("Reset changes")
    .submit_label("Update draft")
    .on_submit(Event::Submit)
```
