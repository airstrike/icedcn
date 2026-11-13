```rust
let questions = vec![
    item("direction", "What should the agent build next?")
        .required(true)
        .description("Choose a direction or describe another task.")
        .choices([
            choice("tool-calls", "Tool call timeline")
                .description("Show what the agent ran and what came back."),
            choice("approvals", "Approval checkpoints")
                .description("Ask before sensitive or destructive actions."),
            choice("handoffs", "Sub-agent handoffs")
                .description("Make delegated work and results easier to follow."),
        ])
        .input("Another agent feature", "Describe another feature…"),
    item("signals", "What should every progress update include?")
        .multiple(true)
        .description("Select all that apply, or skip this question.")
        .choices([
            choice("progress", "Progress"),
            choice("decisions", "Decisions"),
            choice("risks", "Risks"),
            choice("next-step", "Next step"),
        ]),
    item("timing", "When should work begin?")
        .required(true)
        .description("Choose when the agent should begin the work.")
        .choices([
            choice("now", "Start now"),
            choice("next-cycle", "Next development cycle"),
            choice("backlog", "Add it to the backlog"),
        ]),
];
questionnaire(questions)
    .allow_skip(true)
    .shortcuts(Shortcuts::Letters)
    .submit_label("Save plan")
    .on_submit(Event::Submit)
```
