```rust
use browser::icon;
use iced::{Fill, Widget};
use icedxdy::{card, Size, Slot, Theme};
use icedxdy::widget::{button, collapsible, column, row, tabs};

enum Entry {
    File(&'static str),
    Folder(&'static str, &'static [Entry]),
}

const FILE_TREE: &[Entry] = &[
    Entry::Folder("components", &[
        Entry::Folder("ui", &[Entry::File("button.tsx"), Entry::File("card.tsx")]),
        Entry::File("login-form.tsx"),
    ]),
    Entry::File("app.tsx"),
];

fn entry<Message: Clone + 'static>(node: &'static Entry) -> Slot<'static, Message> {
    match node {
        Entry::File(name) => button(row![icon::file(), *name].spacing(8).width(Fill))
            .size(Size::Sm)
            .style(button::Link)
            .width(Fill)
            .into(),
        Entry::Folder(name, children) => collapsible(move |folder| {
            let chevron = if folder.is_open() {
                icon::chevron_down()
            } else {
                icon::chevron_right()
            };

            column![
                folder.trigger(
                    button(row![chevron, icon::folder(), *name].spacing(8).width(Fill))
                        .size(Size::Sm)
                        .style(button::Ghost)
                        .width(Fill),
                ),
                folder.content(
                    column(children.iter().map(entry))
                        .width(Fill)
                        .spacing(4)
                        .padding(iced::Padding::ZERO.top(4).left(20)),
                ),
            ]
            .width(Fill)
            .into()
        })
        .width(Fill)
        .into(),
    }
}

fn file_tree<Message: Clone + 'static>() -> iced::Element<'static, Message, Theme> {
    let header = tabs(|tabs| {
        tabs.list([
            tabs.trigger("explorer", "Explorer"),
            tabs.trigger("settings", "Outline"),
        ])
    })
    .default_value("explorer");

    card(
        column![
            card::header(Slot::from(header)),
            card::content(column(FILE_TREE.iter().map(entry)).width(Fill).spacing(4)).boxed(),
        ]
        .width(Fill)
        .padding([12, 0])
        .spacing(8),
    )
    .width(Fill.max(256.0))
    .boxed()
}
```
