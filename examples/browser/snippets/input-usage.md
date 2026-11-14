## Usage

```rust
use iced::Fill;
use icedcn::widget::input;

input("Enter text").width(Fill)
```

Use `field(input("Email")).label("Email address")` to give the input a clickable label. Add helper text with `.description(...)`. An uncontrolled input keeps what you type. For an application-owned value, use `.value(value).on_input(Message::Edited)` and store the new text in `update`. Use `.disabled(true)` to disable the control.

The browser examples use 14px input text on desktop and `.text_size(16.)` below a 768px window width.
