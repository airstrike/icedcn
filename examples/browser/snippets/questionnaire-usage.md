## Usage

Define the ordered questions once and handle the submitted answers in your application.

```rust
use icedxdy::widget::questionnaire;
use icedxdy::widget::questionnaire::{choice, item};

questionnaire([
    item("direction", "What should we prototype next?")
        .required(true)
        .choices([choice("delegation", "Delegation"), choice("questions", "Question prompts")])
        .input("Another answer", "Type another answer…"),
])
.on_submit(Event::Submit)
```
