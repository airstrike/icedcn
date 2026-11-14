## Usage

```rust
use icedxdy::widget::input;
```

```rust
input("Enter text")
```

Use `field(input("Email")).label("Email address")` to give the input a clickable label. Add helper text with `.description(...)`. An uncontrolled input keeps what you type. Use `.on_input(...)` to receive edits and `.disabled(true)` to disable the control.
