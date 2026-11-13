```rust
let questions = vec![
    item("scope", "How large is the change?")
        .required(true)
        .choices([
            choice("small", "Small patch"),
            choice("medium", "Feature-sized change"),
            choice("large", "Cross-package change"),
        ]),
    item("strategy", "How should commits be organized?")
        .required(true)
        .choices([
            choice("single", "Single commit"),
            choice("logical", "Logical commits"),
            choice("squash", "Squash before review"),
        ]),
    item("tests", "Which tests should run?")
        .required(true)
        .choices([
            choice("targeted", "Targeted tests"),
            choice("package", "Package suite"),
            choice("workspace", "Full workspace"),
        ]),
    item("delivery", "How should the work be delivered?")
        .required(true)
        .choices([
            choice("patch", "Patch only"),
            choice("commit", "Committed locally"),
            choice("branch", "Push a review branch"),
        ]),
];
questionnaire(questions)
    .progress(render_progress)
    .submit_label("Finish plan")
    .on_submit(Event::Submit)
```
