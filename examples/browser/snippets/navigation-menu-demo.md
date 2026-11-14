```rust
use icedxdy::widget::{container, menu, navigation_menu};

let mut entries = vec![
    navigation_menu::menu("Getting started", || {
        column![
            doc_link(
                "Introduction",
                "Reusable widgets for iced applications.",
                "https://github.com/airstrike/icedxdy"
            ),
            doc_link(
                "Installation",
                "How to install dependencies and structure your app.",
                "https://github.com/airstrike/icedxdy"
            ),
            doc_link(
                "Typography",
                "Styles for headings, paragraphs, lists...etc",
                "icedxdy://component/typography"
            )
        ]
        .into()
    })
    .width(384),
];
if wide {
    entries.push(navigation_menu::menu("Components", || column![row![container(doc_link("Alert Dialog","A modal dialog that interrupts the user with important content and expects a response.","icedxdy://component/alert-dialog")).width(iced::Fill),container(doc_link("Hover Card","For sighted users to preview content available behind a link.","icedxdy://component/hover-card")).width(iced::Fill)].spacing(8),row![container(doc_link("Progress","Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.","icedxdy://component/progress")).width(iced::Fill),container(doc_link("Scroll Area","Scroll through content inside a fixed viewport.","icedxdy://component/scroll-area")).width(iced::Fill)].spacing(8),row![container(doc_link("Tabs","A set of layered sections of content—known as tab panels—that are displayed one at a time.","icedxdy://component/tabs")).width(iced::Fill),container(doc_link("Tooltip","A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.","icedxdy://component/tooltip")).width(iced::Fill)].spacing(8)].spacing(8).into()).width(500));
}
entries.push(navigation_menu::link(
    "Docs",
    Message::OpenUrl("https://github.com/airstrike/icedxdy"),
));
navigation_menu(entries)
```
