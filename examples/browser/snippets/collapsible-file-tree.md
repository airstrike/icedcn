```rust
use crate::icon;
use iced::alignment::Vertical;
use iced::{Fill, Padding};
use icedcn::theme::tweak::{Fg, Token};
use icedcn::widget::{button, collapsible, column, row, tabs};
use icedcn::{Size, Slot, card};

enum Entry {
    File(&'static str),
    Folder(&'static str, &'static [Entry]),
}

const FILE_TREE: &[Entry] = &[
    Entry::Folder(
        "components",
        &[
            Entry::Folder(
                "ui",
                &[
                    Entry::File("button.rs"),
                    Entry::File("card.rs"),
                    Entry::File("dialog.rs"),
                    Entry::File("input.rs"),
                    Entry::File("select.rs"),
                    Entry::File("table.rs"),
                ],
            ),
            Entry::File("login_form.rs"),
            Entry::File("register_form.rs"),
        ],
    ),
    Entry::Folder(
        "lib",
        &[
            Entry::File("util.rs"),
            Entry::File("style.rs"),
            Entry::File("api.rs"),
        ],
    ),
    Entry::Folder(
        "state",
        &[
            Entry::File("window_size.rs"),
            Entry::File("debounce.rs"),
            Entry::File("storage.rs"),
        ],
    ),
    Entry::Folder(
        "types",
        &[Entry::File("types.rs"), Entry::File("request.rs")],
    ),
    Entry::Folder(
        "public",
        &[
            Entry::File("favicon.ico"),
            Entry::File("logo.svg"),
            Entry::File("images"),
        ],
    ),
    Entry::File("main.rs"),
    Entry::File("layout.rs"),
    Entry::File("theme.rs"),
    Entry::File("Cargo.toml"),
    Entry::File("rustfmt.toml"),
    Entry::File("README.md"),
    Entry::File(".gitignore"),
];

fn entry<'a, Message: Clone + 'static>(node: &'a Entry) -> Slot<'a, Message> {
    match node {
        Entry::File(name) => button(
            row![icon::file().size(14).line_height(1.0), *name,]
                .spacing(8)
                .align_y(Vertical::Center)
                .width(Fill),
        )
        .size(Size::Sm)
        .style(button::Link + Fg(Token::Foreground))
        .width(Fill)
        .into(),
        Entry::Folder(name, children) => collapsible(move |folder| {
            column![
                folder.trigger(
                    button(
                        row![
                            folder.indicator().size(14),
                            icon::folder().size(14).line_height(1.0),
                            *name,
                        ]
                        .spacing(4)
                        .align_y(Vertical::Center)
                        .width(Fill)
                    )
                    .size(Size::Sm)
                    .style(button::Ghost)
                    .animate_hover(false)
                    .width(Fill)
                ),
                folder.content(
                    column(children.iter().map(entry))
                        .width(Fill)
                        .spacing(4)
                        .padding(Padding::ZERO.top(4).left(20))
                ),
            ]
            .width(Fill)
            .into()
        })
        .width(Fill)
        .into(),
    }
}

fn file_tree<'a, Message: Clone + 'static>() -> Slot<'a, Message> {
    let header = tabs(|tabs| {
        tabs.list([
            tabs.trigger("explorer", "Explorer"),
            tabs.trigger("settings", "Outline"),
        ])
    })
    .default_value("explorer")
    .width(Fill)
    .list_width(Fill);
    card(
        column![
            card::header(Slot::from(header)).inset(12.),
            card::content(column(FILE_TREE.iter().map(entry)).width(Fill).spacing(4))
                .padding([0, 12]),
        ]
        .width(Fill)
        .padding([12, 0])
        .spacing(8),
    )
    .gap(0)
    .padding(0)
    .width(Fill.max(256))
    .into()
}

fn view<'a>() -> Slot<'a, ()> {
    file_tree()
}
```
