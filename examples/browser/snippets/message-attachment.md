```rust
use crate::icon;
use iced::alignment::Horizontal::Right;
use iced::widget::image;
use iced::{ContentFit, Widget};
use icedcn::Size;
use icedcn::widget::bubble::Muted;
use icedcn::widget::{attachment, bubble, button, column, message, tooltip};
use std::sync::LazyLock;

static WORKSPACE: LazyLock<image::Handle> = LazyLock::new(|| {
    image::Handle::from_bytes(
        include_bytes!("../../assets/attachments/workspace.jpg").as_slice(),
    )
});
let cover = image(WORKSPACE.clone())
    .width(80)
    .height(80)
    .content_fit(ContentFit::Cover)
    .border_radius(8);
let download = tooltip(
    attachment::action(icon::download())
        .size(Size::Sm)
        .style(button::Secondary)
        .on_press(Message::SaveAttachment),
    "Download",
);
frame(
    column![
        message(
            column![
                attachment("").vertical(true).width(96).media(cover.boxed()),
                bubble(
                    "Here's the image. Can you add it to the PDF? Use it for the cover page."
                )
            ]
            .align_x(Right)
            .spacing(10)
        )
        .align(Right),
        message(
            column![
                bubble("Done. Here's the PDF with the image added as the cover page.")
                    .style(Muted),
                attachment("sales-dashboard.pdf")
                    .width(250)
                    .description("PDF · 2.4 MB")
                    .media(icon::file_text())
                    .action(download)
            ]
            .spacing(10)
        ),
        message(bubble("Thanks. Looks good.")).align(Right)
    ]
    .spacing(32),
)
```
