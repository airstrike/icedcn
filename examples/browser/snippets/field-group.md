```rust
use iced::widget::{rich_text, span};
use iced::{Fill, Widget};
use icedcn::Horizontal;
use icedcn::Slot;
use icedcn::theme::text::muted;
use icedcn::widget::{checkbox, column, field, separator, text};

fn notifications<'a, M: Clone + 'a>(manage: M) -> Slot<'a, M> {
    let description = rich_text([
        span("Get notified when tasks you've created have updates. "),
        span("Manage tasks").link(()).underline(true),
    ])
    .size(14)
    .line_height(1.5)
    .width(Fill)
    .style(|theme| muted(theme).into())
    .on_link_click(move |()| manage.clone())
    .boxed();
    let notifications = field::group(["Push notifications", "Email notifications"].map(|name| {
        field(checkbox())
            .orientation(Horizontal)
            .label(text::small(name))
            .into()
    }))
    .spacing(12);
    field::group([
    column![field::label("Responses"),
        text::muted("Get notified when ChatGPT responds to requests that take time, like research or image generation.").line_height(1.5),
    ].width(Fill).spacing(8).into(),
    field(checkbox().default_checked(true).disabled(true)).orientation(Horizontal).label(text::muted("Push notifications")).into(),
    separator().into(),
    column![field::label("Tasks"), description].width(Fill).spacing(8).into(),
    notifications.into(),
]).width(320).into()
}
```
