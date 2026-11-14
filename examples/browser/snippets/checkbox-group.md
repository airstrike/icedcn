```rust
use iced::{Pixels, Shrink};
use icedcn::widget::{checkbox, column, field, text};

let choices = [
    "Hard disks",
    "External disks",
    "CDs, DVDs, and iPods",
    "Connected servers",
];
field::set(
    column![
        text::label("Show these items on the desktop:").line_height(Pixels(20.0)),
        text::muted("Select the items you want to show on the desktop.")
            .line_height(Pixels(21.0)),
    ]
    .spacing(2),
    None,
    field::group(choices.into_iter().enumerate().map(|(index, label)| {
        field(checkbox().default_checked(index < 2))
            .orientation(field::Horizontal)
            .label(text::small(label).line_height(Pixels(19.25)))
            .into()
    }))
    .spacing(12),
)
.width(Shrink)
```
