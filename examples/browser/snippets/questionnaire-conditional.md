```rust
use icedcn::widget::questionnaire;
use icedcn::widget::questionnaire::{choice, item};

let questions = vec![
    item("runtime", "Where should the agent run?")
        .required(true)
        .description("Cloud runs add an environment question to this flow.")
        .choices([
            choice("local", "Local workspace"),
            choice("cloud", "Cloud workspace"),
        ]),
    item("environment", "Which cloud environment should it use?")
        .required(true)
        .choices([
            choice("preview", "Preview"),
            choice("staging", "Staging"),
            choice("isolated", "Isolated sandbox"),
        ])
        .enabled_when(|answers| answers.get("runtime").is_some_and(|a| a.contains("cloud"))),
    item("approval", "When should the agent request approval?")
        .required(true)
        .choices([
            choice("writes", "Before writing files"),
            choice("commands", "Before running commands"),
            choice("sensitive", "Only for sensitive actions"),
        ]),
];
questionnaire(questions)
    .submit_label("Save execution plan")
    .on_submit(Event::Submit)
```
