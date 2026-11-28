```rust
use icedcn::Horizontal;
use icedcn::View;
use icedcn::widget::{checkbox, field, separator, text};

fn devices<'a, M: Clone + 'a>() -> View<'a, M> {
    let devices = [
        "Hard disks",
        "External disks",
        "CDs, DVDs, and iPods",
        "Connected servers",
    ];
    let items = field::group(devices.map(|name| {
        field(checkbox())
            .orientation(Horizontal)
            .label(text::small(name))
            .into()
    }))
    .spacing(12);
    field::group([
    field::set(field::label("Show these items on the desktop"),
        Some("Select the items you want to show on the desktop."), items).into(),
    separator().into(),
    field(checkbox().default_checked(true)).orientation(Horizontal).label("Sync Desktop & Documents folders")
        .description("Your Desktop & Documents folders are being synced with iCloud Drive. You can access them from other devices.").into(),
]).width(320).into()
}
```
