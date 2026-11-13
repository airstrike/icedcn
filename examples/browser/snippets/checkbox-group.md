```rust
pub(super) fn desktop_items<'a>(checked: bool) -> Slot<'a, Message> {
    let choices = ["Hard disks", "External disks", "CDs, DVDs, and iPods", "Connected servers"];
    let legend = if checked { "Show these items on the desktop:" } else { "Show these items on the desktop" };
    field::set(text::label(legend), Some("Select the items you want to show on the desktop."),
        field::group(choices.into_iter().enumerate().map(|(index, label)|
            field(checkbox().default_checked(checked && index < 2)).orientation(Horizontal).label(text::small(label)).into()
        )).spacing(12)).into()
}

desktop_items(true)
```
