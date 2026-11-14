```rust
use iced::{ContentFit, Widget};
use iced::widget::image;
use icedxdy::Slot;
use icedxdy::widget::{attachment, tooltip};

let preview = image(workspace).width(40).height(40).content_fit(ContentFit::Cover);
let files: [(_, _, Slot<'_, Message>); 4] = [
    ("briefing-notes.pdf", "PDF · 1.4 MB", icon::file_text().into()),
    ("workspace.png", "PNG · 820 KB", preview.boxed().into()),
    ("customers.csv", "CSV · 18 KB", icon::table().into()),
    ("renderer.tsx", "TSX · 12 KB", icon::file_code().into()),
];

attachment::group(files.into_iter().map(|(name, metadata, media)| {
    attachment(name)
        .media(media)
        .description(metadata)
        .width(256)
        .action(tooltip(
            attachment::action(icon::x()).on_press(Message::Remove(name)),
            format!("Remove {name}"),
        ))
        .into()
}))
```
