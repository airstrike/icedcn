```rust
use iced::Fill;
use icedcn::widget::container;
use icedcn::widget::{attachment, dialog, tooltip};

let name = "research-summary.pdf";
container(dialog(
move |d| {
let opener = *d;
attachment(name)
.media(icon::file_search().size(16).selectable(false))
.description("Open preview dialog")
.trigger(move |body| {
opener.trigger(attachment::trigger(body))
})
.action(tooltip(
attachment::action(icon::copy().size(12).selectable(false)).on_press(Message::Copy),
"Copy link",
))
.action(tooltip(
attachment::action(icon::x().size(12).selectable(false)).on_press(Message::Remove(name)),
format!("Remove {name}"),
))
.into()
},
move |_| dialog::header(name, "The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it."),
)
.max_width(448.))
.width(Fill).max_width(384).padding([48, 0])
```
