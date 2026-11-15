```rust
use icedcn::{
    Slot,
    widget::{button, button_group, kbd, kbd_group, row, text, tooltip},
};

pub fn view<'a>() -> Slot<'a, ()> {
    button_group![
        tooltip(
            button("Save").style(button::Outline),
            row![text::xs("Save Changes"), kbd("S").style(kbd::Tooltip)].spacing(4)
        ),
        tooltip(
            button("Print").style(button::Outline),
            row![
                text::xs("Print Document"),
                kbd_group![
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
