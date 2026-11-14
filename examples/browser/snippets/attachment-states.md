```rust
use icedxdy::widget::attachment::State;
use icedxdy::Slot;
use icedxdy::widget::{attachment, column, spinner, tooltip};

let files: [(_, _, _, Slot<'_, Message>); 5] = [
    ("selected-file.pdf", "Ready to upload", State::Idle, icon::clock().into()),
    ("design-system.zip", "Uploading · 64%", State::Uploading, spinner().size(16.0).into()),
    ("market-research.pdf", "Processing document", State::Processing, icon::file_text().into()),
    ("financial-model.xlsx", "Upload failed. Try again.", State::Error, icon::file_warning().into()),
    ("uploaded-report.pdf", "Uploaded · 1.8 MB", State::Done, icon::check().into()),
];

column(files.into_iter().map(|(name, description, state, media)| {
    let mut card = attachment(name).media(media).description(description).state(state);
    if state == State::Error {
        card = card.action(tooltip(
            attachment::action(icon::refresh_ccw()).on_press(Message::Retry),
            "Retry upload",
        ));
    }
    let label = if state == State::Uploading { "Cancel upload".into() } else { format!("Remove {name}") };
    card.action(tooltip(
        attachment::action(icon::x()).on_press(Message::Remove(name)),
        label,
    )).into()
})).spacing(8)
```
