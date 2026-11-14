```rust
use iced::Fill;
use icedxdy::widget::{attachment, button, dialog, tooltip};

let name = "research-summary.pdf";
dialog(
    move |d| {
        let opener = *d;
        attachment(name)
            .media(icon::file_search())
            .description("Open preview dialog")
            .trigger(move |body| {
                opener.trigger(button(body).style(button::Ghost).height(40).width(Fill))
            })
            .action(tooltip(
                attachment::action(icon::copy()).on_press(Message::Copy),
                "Copy link",
            ))
            .action(tooltip(
                attachment::action(icon::x()).on_press(Message::Remove(name)),
                format!("Remove {name}"),
            ))
            .into()
    },
    move |_| dialog::header(name, "The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it."),
)
.max_width(448.)
```
