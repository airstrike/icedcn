## Usage

```rust
use icedxdy::widget::input;
```

```rust
input("Enter text")
```

Use `field(input("Email")).label("Email address")` to give the input a clickable label. Add helper text with `.description(...)`. An uncontrolled input keeps what you type. For an application-owned value, use `.value(value).on_input(Message::Edited)` and store the new text in `update`. Use `.disabled(true)` to disable the control.
