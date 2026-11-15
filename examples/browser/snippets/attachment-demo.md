```rust
use icedcn::core::oklch;
use icedcn::{Color, Theme};

fn canvas(theme: &Theme) -> Color {
    if theme.neutral.is_dark {
        theme.neutral.base
    } else {
        oklch(0.98, 0.0, 0.0)
    }
}
use iced::widget::image;
use iced::{ContentFit, Widget};
use icedcn::widget::attachment::State;
use icedcn::widget::container;
use icedcn::widget::{attachment, column, spinner, tooltip};

let photos = [
    ("workspace.png", "PNG · 820 KB", workspace),
    ("desk-reference.jpg", "JPG · 1.1 MB", desk),
    ("office-reference.jpg", "JPG · 940 KB", office),
];

let previews = attachment::group(photos.into_iter().map(|(name, metadata, handle)| {
    let picture = image(handle)
        .width(102)
        .height(102)
        .content_fit(ContentFit::Cover)
        .border_radius(10);
    attachment(name)
        .vertical(true)
        .media(picture.boxed())
        .description(metadata)
        .into()
}))
.fade_to(canvas);

container(
    column![
        previews,
        attachment("sales-dashboard.pdf")
            .media(spinner().size(16.0))
            .state(State::Uploading)
            .description("Uploading · 64%")
            .action(tooltip(
                attachment::action(icon::x().size(12).selectable(false))
                    .on_press(Message::Remove("sales-dashboard.pdf")),
                "Cancel upload",
            )),
        attachment("message_renderer.rs")
            .media(icon::file_code().size(16).selectable(false))
            .description("Rust · 12 KB")
            .action(tooltip(
                attachment::action(icon::x().size(12).selectable(false))
                    .on_press(Message::Remove("message_renderer.rs")),
                "Remove message_renderer.rs",
            )),
    ]
    .spacing(12)
    .width(iced::Fill),
)
.width(iced::Fill)
.max_width(384)
.padding([48, 0])
```
