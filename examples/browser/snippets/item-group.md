```rust
use crate::icon;
use iced::Fill;
use iced::widget::image;
use icedcn::widget::{avatar, button, column, item};

fn people() -> [(&'static str, &'static str, image::Handle); 3] {
    [
        ("Alex Chen", "alex@example.com", alex_handle()),
        ("maxleiter", "maxleiter@example.com", maxleiter_handle()),
        ("evilrabbit", "evilrabbit@example.com", evilrabbit_handle()),
    ]
}

column(people().map(|(name, email, portrait)| {
    item::item([
        item::media(avatar(Some(portrait)).fallback(&name[..1])).into(),
        item::content([item::title(name).into(), item::description(email).into()])
            .spacing(4)
            .into(),
        item::actions([button(icon::plus())
            .style(button::Ghost)
            .square()
            .rounded()
            .into()]),
    ])
    .style(item::Outline)
    .into()
}))
.max_width(384.0)
.width(Fill)
```
