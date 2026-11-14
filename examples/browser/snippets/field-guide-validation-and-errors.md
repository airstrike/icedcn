## Validation and Errors

Give each error a specific instruction. Set the invalid style and attach the explanation to the same field. Keep the entered value available for correction. The payment and profile examples show required-field errors after a blank submission and clear them as you type.

```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::theme::text_input::Invalid;
use icedcn::widget::{field, input};

fn username(value: String, unavailable: bool) -> Slot<'static, ()> {
    let control = input("Username").width(Fill).default_value(value);
    if unavailable {
        field(control.style(Invalid)).label("Username").error("Choose another username.").into()
    } else {
        field(control).label("Username").into()
    }
}
```
