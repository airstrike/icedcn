```rust
use crate::icon;
use icedcn::View;
use icedcn::widget::attachment::State;
use icedcn::widget::{attachment, column, container, spinner, tooltip};

let files: [(_, _, _, View<'_, Message>); 5] = [
    (
        "selected-file.pdf",
        "Ready to upload",
        State::Idle,
        icon::clock().size(16).selectable(false).into(),
    ),
    (
        "design-system.zip",
        "Uploading · 64%",
        State::Uploading,
        spinner().size(16.0).into(),
    ),
    (
        "market-research.pdf",
        "Processing document",
        State::Processing,
        icon::file_text().size(16).selectable(false).into(),
    ),
    (
        "financial-model.xlsx",
        "Upload failed. Try again.",
        State::Error,
        icon::file_warning().size(16).selectable(false).into(),
    ),
    (
        "uploaded-report.pdf",
        "Uploaded · 1.8 MB",
        State::Done,
        icon::check().size(16).selectable(false).into(),
    ),
];

container(
    column(files.into_iter().map(|(name, description, state, media)| {
        let mut card = attachment(name)
            .media(media)
            .description(description)
            .state(state);
        if state == State::Error {
            card = card.action(tooltip(
                attachment::action(icon::refresh_cw().size(12).selectable(false))
                    .on_press(Message::Retry),
                "Retry upload",
            ));
        }
        let label = if state == State::Uploading {
            "Cancel upload".into()
        } else {
            format!("Remove {name}")
        };
        card.action(tooltip(
            attachment::action(icon::x().size(12).selectable(false))
                .on_press(Message::Remove(name)),
            label,
        ))
        .into()
    }))
    .spacing(8)
    .width(iced::Fill),
)
.width(iced::Fill)
.max_width(384)
.padding([48, 0])
```

The installed preview handles `Message::Remove(name)` by excluding that file from its local list. `Message::Retry` changes financial-model.xlsx from `Error` to `Done` and its description to “Uploaded”. Keep those values in the application component that owns the upload; the attachment only presents the supplied state.
