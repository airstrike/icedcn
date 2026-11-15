```rust
use iced::alignment::Vertical;
use iced::widget::svg;
use iced::{Fill, Padding};
use icedcn::theme::tweak::{Fg, Token};
use icedcn::widget::{button, collapsible, column, row, tabs};
use icedcn::{Size, Slot, Theme, card};

fn icon(path: &'static str, size: f32) -> svg::Svg<'static, Theme> {
    svg(svg::Handle::from_memory(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    ).into_bytes()))
    .width(size).height(size)
    .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}


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

fn entry<Message: Clone + 'static>(node: &'static Entry) -> Slot<'static, Message> {
    match node {
        Entry::File(name) => button(row![
            icon(r#"<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/>"#, 14.),
            *name,
        ]
        .spacing(8)
        .align_y(Vertical::Center)
        .width(Fill))
        .size(Size::Sm)
        .style(button::Link + Fg(Token::Foreground))
        .width(Fill)
        .into(),
        Entry::Folder(name, children) => collapsible(move |folder| {
            column![
                folder.trigger(button(row![
                    folder.indicator().size(14),
                    icon(r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#, 14.),
                    *name,
                ]
                .spacing(4)
                .align_y(Vertical::Center)
                .width(Fill))
                .size(Size::Sm)
                .style(button::Ghost)
                .animate_hover(false)
                .width(Fill)),
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

fn file_tree<Message: Clone + 'static>() -> Slot<'static, Message> {
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


fn view() -> Slot<'static, ()> {
    file_tree()
}
```
