```rust
use iced::Fill;
use icedcn::{
    Slot,
    widget::{button, field, input, sheet, text},
};
const DESCRIPTION: &str = "Make changes to your profile here. Click save when you're done.";
pub fn view() -> Slot<'static, ()> {
    sheet(
        |s| s.trigger(button("Open").style(button::Outline)),
        |s| {
            let entry = |label, value| {
                let control = input("").default_value(value).width(Fill);
                field(control).spacing(12)
                    .label(text::label(label).height(14))
                    .width(Fill)
                    .into()
            };
            sheet::content(
                sheet::header("Edit profile", DESCRIPTION),
                field::group([
                    entry("Name", "Pedro Duarte"),
                    entry("Username", "@peduarte"),
                ])
                .spacing(24)
                .width(Fill),
                [
                    button("Save changes").into(),
                    s.close(button("Close").style(button::Outline)),
                ],
            )
        },
    )
    .into()
}
```
