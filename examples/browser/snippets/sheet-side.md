```rust
use iced::Fill;
use icedcn::{
    Slot, theme,
    widget::{button, column, row, scrollable, sheet, text},
};
const DESCRIPTION: &str = "Make changes to your profile here. Click save when you're done.";
const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
pub fn view<'a>() -> Slot<'a, ()> {
    use sheet::Side::{Bottom, Left, Right, Top};
    row![
        side(Top, "Top"),
        side(Right, "Right"),
        side(Bottom, "Bottom"),
        side(Left, "Left")
    ]
    .wrap()
    .spacing(8)
    .into()
}
fn side(edge: sheet::Side, label: &'static str) -> Slot<'static, ()> {
    sheet(
        move |s| s.trigger(button(label).style(button::Outline)),
        |s| {
            let paragraphs =
                column((0..10).map(|_| text::small(PARAGRAPH).line_height(1.625).into()))
                    .spacing(8)
                    .padding([0, 0, 8, 0])
                    .width(Fill);
            sheet::content(
                sheet::header("Edit profile", DESCRIPTION),
                scrollable(paragraphs)
                    .height(Fill)
                    .width(Fill)
                    .class(theme::scrollable::Hidden),
                [
                    button("Save changes").into(),
                    s.close(button("Cancel").style(button::Outline)),
                ],
            )
        },
    )
    .side(edge)
    .max_height_ratio(0.5)
    .into()
}
```
