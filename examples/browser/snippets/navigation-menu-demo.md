```rust
use icedxdy::widget::{container, menu, navigation_menu};

let mut entries = vec![
    navigation_menu::menu("Getting started", || {
        column![
            doc_link(
                "Introduction",
                "Re-usable components built with Tailwind CSS.",
                "https://ui.shadcn.com/docs"
            ),
            doc_link(
                "Installation",
                "How to install dependencies and structure your app.",
                "https://ui.shadcn.com/docs/installation"
            ),
            doc_link(
                "Typography",
                "Styles for headings, paragraphs, lists...etc",
                "https://ui.shadcn.com/docs/components/radix/typography"
            )
        ]
        .into()
    })
    .width(384),
];
if wide {
    entries.push(navigation_menu::menu("Components", || column![row![container(doc_link("Alert Dialog","A modal dialog that interrupts the user with important content and expects a response.","https://ui.shadcn.com/docs/components/radix/alert-dialog")).width(iced::Fill),container(doc_link("Hover Card","For sighted users to preview content available behind a link.","https://ui.shadcn.com/docs/components/radix/hover-card")).width(iced::Fill)].spacing(8),row![container(doc_link("Progress","Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.","https://ui.shadcn.com/docs/components/radix/progress")).width(iced::Fill),container(doc_link("Scroll-area","Visually or semantically separates content.","https://ui.shadcn.com/docs/components/radix/scroll-area")).width(iced::Fill)].spacing(8),row![container(doc_link("Tabs","A set of layered sections of content—known as tab panels—that are displayed one at a time.","https://ui.shadcn.com/docs/components/radix/tabs")).width(iced::Fill),container(doc_link("Tooltip","A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.","https://ui.shadcn.com/docs/components/radix/tooltip")).width(iced::Fill)].spacing(8)].spacing(8).into()).width(500));
}
entries.push(navigation_menu::link(
    "Docs",
    Message::OpenUrl("https://ui.shadcn.com/docs"),
));
navigation_menu(entries)
```
