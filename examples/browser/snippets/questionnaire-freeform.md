```rust
let questions = vec![
    item("approach", "How should the agent approach this refactor?")
        .required(true)
        .description("Choose a strategy or write a more specific instruction.")
        .choices([
            choice("incremental", "Make the smallest safe change"),
            choice("module", "Refactor one module at a time"),
            choice("rewrite", "Replace the implementation completely"),
        ])
        .input("Another refactoring approach", "Describe another approach…"),
];
questionnaire(questions)
    .shortcuts(Shortcuts::Letters)
    .submit_label("Use this approach")
    .on_submit(Event::Submit)
```
