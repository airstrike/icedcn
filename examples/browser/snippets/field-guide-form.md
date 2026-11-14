## Form

Keep form values in your application or a local iced `Component`. Use `.on_input(...)` to update a controlled value. Give the input’s `.on_submit(...)` and the submit button the same message so Enter and a click run the same validation.

The payment example accepts a function that builds each required field and a submission message. This constructor makes an editable field and shows its error after an unsuccessful submission.

```rust
use iced::Fill;
use icedcn::theme::text_input::Invalid;
use icedcn::widget::{field, input};

#[derive(Clone)]
enum Event {
    Edit(&'static str, String),
    Submit,
}

fn required(
    name: &'static str,
    hint: &'static str,
    value: &str,
    attempted: bool,
) -> field::Field<'static, Event> {
    let control = input(hint)
        .width(Fill)
        .value(value.to_owned())
        .on_input(move |value| Event::Edit(name, value))
        .on_submit(Event::Submit);
    if attempted && value.trim().is_empty() {
        field(control.style(Invalid)).label(name).error(format!("{name} is required."))
    } else {
        field(control).label(name)
    }
}
```
