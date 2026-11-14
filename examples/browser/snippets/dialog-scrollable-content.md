```rust
use iced::{Shrink, Widget};
use iced::widget::responsive;
use icedcn::Slot;
use icedcn::widget::{button, column, dialog, scrollable, text};

const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

dialog(
    |d| d.trigger(button("Scrollable Content").style(button::Outline)),
    |d| {
        let content = responsive(|size| {
            let paragraphs = column((0..10).map(|_| text(PARAGRAPH).into())).spacing(16);
            Slot::from(scrollable(paragraphs).height((size.height + 64.0) / 2.0)).compose()
        }).height(Shrink).boxed();
        column![
            dialog::header("Scrollable Content", "This is a dialog with scrollable content."),
            content,
        ].spacing(16).into()
    },
)
```
