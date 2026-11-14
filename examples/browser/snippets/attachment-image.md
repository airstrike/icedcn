```rust
use iced::{ContentFit, Fill, Widget};
use iced::widget::{button, image};
use icedxdy::theme::button::ghost;
use icedxdy::widget::{attachment, tooltip};

let photos = [
    ("workspace.png", "PNG · 820 KB", workspace),
    ("desk-reference.jpg", "JPG · 1.1 MB", desk),
    ("office-reference.jpg", "JPG · 940 KB", office),
];

attachment::group(photos.into_iter().map(|(name, metadata, handle)| {
    let picture = image(handle).width(104).height(104)
        .content_fit(ContentFit::Cover).border_radius(8);
    attachment(name)
        .vertical(true)
        .media(picture.boxed())
        .description(metadata)
        .trigger(move |body| {
            button(body.compose())
                .width(Fill)
                .padding(0)
                .style(|theme, status| ghost(theme, status.into()).into())
                .on_press(Message::Open(name))
                .boxed().into()
        })
        .action(tooltip(
            attachment::action(icon::x()).on_press(Message::Remove(name)),
            format!("Remove {name}"),
        ))
        .into()
}))
```
