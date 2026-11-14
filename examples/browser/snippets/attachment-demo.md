```rust
use iced::{ContentFit, Widget};
use iced::widget::image;
use icedxdy::widget::{attachment, column, spinner, tooltip};
use icedxdy::widget::attachment::State;

let photos = [
    ("workspace.png", "PNG · 820 KB", workspace),
    ("desk-reference.jpg", "JPG · 1.1 MB", desk),
    ("office-reference.jpg", "JPG · 940 KB", office),
];

let previews = attachment::group(photos.into_iter().map(|(name, metadata, handle)| {
    let picture = image(handle).width(104).height(104)
        .content_fit(ContentFit::Cover).border_radius(8);
    attachment(name)
        .vertical(true)
        .media(picture.boxed())
        .description(metadata)
        .into()
}));

column![
    previews,
    attachment("sales-dashboard.pdf")
        .media(spinner().size(16.0))
        .state(State::Uploading)
        .description("Uploading · 64%")
        .action(tooltip(
            attachment::action(icon::x()).on_press(Message::Remove("sales-dashboard.pdf")),
            "Cancel upload",
        )),
    attachment("message-renderer.tsx")
        .media(icon::file_code())
        .description("TypeScript · 12 KB")
        .action(tooltip(
            attachment::action(icon::x()).on_press(Message::Remove("message-renderer.tsx")),
            "Remove message-renderer.tsx",
        )),
].spacing(12)
```
