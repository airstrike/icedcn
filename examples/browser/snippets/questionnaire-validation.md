```rust
use icedcn::widget::questionnaire;
use icedcn::widget::questionnaire::{choice, item};

let questions = vec![
    item("detail", "How much detail should the answer include?")
        .required(true)
        .description("Choose the response depth.")
        .choices([
            choice("summary", "Concise summary"),
            choice("complete", "Complete answer"),
        ]),
    item("audience", "Who will read the answer?")
        .required(true)
        .description("Public answers require complete context.")
        .choices([
            choice("team", "My team"),
            choice("public", "Public audience"),
        ]),
];
questionnaire(questions)
    .validate(validate_answers)
    .submit_label("Validate answers")
    .on_submit(Event::Submit)
```
