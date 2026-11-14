```rust
use icedxdy::{Color, Theme};
use icedxdy::core::oklch;

fn canvas(theme: &Theme) -> Color {
    if theme.neutral.is_dark { theme.neutral.base } else { oklch(0.98, 0.0, 0.0) }
}
use icedxdy::widget::container;
use iced::{ContentFit, Widget};
use iced::widget::image;
use icedxdy::Slot;
use icedxdy::widget::{attachment, tooltip};

let preview = image(workspace).width(40).height(40).content_fit(ContentFit::Cover).border_radius(10);
let files: [(_, _, Slot<'_, Message>); 4] = [
    ("briefing-notes.pdf", "PDF · 1.4 MB", icon::file_text().size(16).selectable(false).into()),
    ("workspace.png", "PNG · 820 KB", preview.boxed().into()),
    ("customers.csv", "CSV · 18 KB", icon::table().size(16).selectable(false).into()),
    ("renderer.tsx", "TSX · 12 KB", icon::file_code().size(16).selectable(false).into()),
];

container(attachment::group(files.into_iter().map(|(name, metadata, media)| {
    attachment(name)
        .media(media)
        .description(metadata)
        .width(256)
        .action(tooltip(
            attachment::action(icon::x().size(12).selectable(false)).on_press(Message::Remove(name)),
            format!("Remove {name}"),
        ))
        .into()
})).fade_to(canvas))
    .width(iced::Fill).max_width(384).padding([48, 0])
```
