```rust
use icedcn::{Color, Theme};
use icedcn::core::oklch;

fn canvas(theme: &Theme) -> Color {
    if theme.neutral.is_dark { theme.neutral.base } else { oklch(0.98, 0.0, 0.0) }
}
use icedcn::widget::container;
use iced::{ContentFit, Fill, Widget};
use iced::widget::image;
use icedcn::widget::{attachment, tooltip};

let photos = [
    ("workspace.png", "PNG · 820 KB", workspace, "https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80"),
    ("desk-reference.jpg", "JPG · 1.1 MB", desk, "https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80"),
    ("office-reference.jpg", "JPG · 940 KB", office, "https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80"),
];

container(attachment::group(photos.into_iter().map(|(name, metadata, handle, url)| {
    let picture = image(handle).width(102).height(102)
        .content_fit(ContentFit::Cover).border_radius(10);
    attachment(name)
        .vertical(true)
        .media(picture.boxed())
        .description(metadata)
        .trigger(move |target| attachment::trigger(target).on_press(Message::Open(url)).into())
        .action(tooltip(
            attachment::action(icon::x().size(12).selectable(false)).on_press(Message::Remove(name)),
            format!("Remove {name}"),
        ))
        .into()
})).fade_to(canvas))
    .width(iced::Fill).max_width(384).padding([48, 0])
```
