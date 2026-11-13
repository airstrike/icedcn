```rust
button_group![
    group::decorated(button("Save").style(button::Outline), |button| tooltip(button,
        row![text::xs("Save Changes"), kbd("S").style(kbd::Tooltip)].spacing(4)).into()),
    group::decorated(button("Print").style(button::Outline), |button| tooltip(button,
        row![text::xs("Print Document"), kbd_group![kbd("Ctrl").style(kbd::Tooltip), kbd("P").style(kbd::Tooltip)]].spacing(4)).into())
]
```
