```rust
popover(|&p| button_group![
    button![icon::bot(), "Copilot"].style(button::Outline),
    group::decorated(button(icon::chevron_down()).square().style(button::Outline), move |b| p.trigger(b))
].into(), |_| column![
    popover::header("Start a new task with Copilot", "Describe your task in natural language."),
    field(textarea("I need to...")).description("Copilot will open a pull request for review.")
].spacing(12).width(Fill).into()).align(popover::Align::End)
```
