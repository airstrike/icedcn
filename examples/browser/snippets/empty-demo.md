```rust
use crate::icon;
use iced::Center;
use icedcn::Size;
use icedcn::theme::Fg;
use icedcn::widget::{button, column, empty, row};

empty::empty()
    .media(empty::media(icon::folder_code()).style(empty::Icon))
    .title("No Projects Yet")
    .description(
        "You haven't created any projects yet. Get started by creating your first project.",
    )
    .content(
        column![
            row![
                button("Create Project"),
                button("Import Project").style(button::Outline),
            ]
            .spacing(8),
            button!["Learn More", icon::arrow_up_right()]
                .style(button::Link + Fg::MutedForeground)
                .size(Size::Sm),
        ]
        .spacing(16)
        .align_x(Center),
    )
```
