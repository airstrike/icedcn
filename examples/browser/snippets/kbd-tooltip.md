```rust
use icedcn::{
    View,
    widget::{button, kbd, row, text, tooltip},
};

pub fn view<'a>() -> View<'a> {
    button::group![
        tooltip(
            button("Save").style(button::Outline),
            row![text::xs("Save Changes"), kbd("S").style(kbd::Tooltip)].spacing(4)
        ),
        tooltip(
            button("Print").style(button::Outline),
            row![
                text::xs("Print Document"),
                kbd::group![
                    kbd("Ctrl").style(kbd::Tooltip),
                    kbd("P").style(kbd::Tooltip)
                ]
            ]
            .spacing(4)
        ),
    ]
    .into()
}
```
