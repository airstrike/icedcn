//! Collapsible sections, controls and a file explorer.

use crate::icon;
use icedcn::theme;
// RTL is omitted because the native application uses left-to-right layout.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::{Fill, Padding, Task};
use icedcn::theme::tweak::{Bg, Fg, Token, border};
use icedcn::widget::{button, collapsible, column, container, input, row, space, tabs, text};
use icedcn::{Size, Slot, Theme, card, radius};

use crate::{Message, page};

pub static META: page::Meta = page::Meta {
    title: "Collapsible",
    description: "An interactive component which expands/collapses a panel.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/collapsible-demo.md",
    },
    page::Block::Md("examples/browser/snippets/collapsible-usage.md"),
    page::Block::Md("examples/browser/snippets/collapsible-composition.md"),
    page::Block::Md("examples/browser/snippets/collapsible-guide-controlled-state.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: None,
        snippet: "examples/browser/snippets/collapsible-basic.md",
    },
    page::Block::Example {
        id: "settings",
        title: "Settings Panel",
        description: Some("Use a trigger button to reveal additional settings."),
        snippet: "examples/browser/snippets/collapsible-settings.md",
    },
    page::Block::Example {
        id: "file-tree",
        title: "File Tree",
        description: Some(
            "Use nested collapsibles to build a file tree. In a conversation, this is the File Tree pattern.",
        ),
        snippet: "examples/browser/snippets/collapsible-file-tree.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[
            page::Block::Md("examples/browser/snippets/collapsible-api-reference.md"),
            page::Block::Md("examples/browser/snippets/collapsible-api-trigger.md"),
            page::Block::Md("examples/browser/snippets/collapsible-api-content.md"),
        ],
    },
];

pub struct CollapsiblePage {
    state: page::State,
}

impl CollapsiblePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn bordered<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Slot<'a, Message> {
    container(content)
        .width(Fill)
        .padding([9, 17])
        .style(
            theme::container::Transparent
                + border::Width(1.0)
                + border::Color(Token::Border)
                + radius(radius::MD),
        )
        .into()
}

fn detail<'a, Message: 'a>(title: &'a str, body: &'a str) -> Slot<'a, Message> {
    bordered(column![
        text::small(title)
            .medium()
            .line_height(LineHeight::Absolute(20.into())),
        text::muted(body).line_height(LineHeight::Absolute(20.into())),
    ])
}

fn demo<'a, Message: Clone + 'static>() -> Slot<'a, Message> {
    collapsible(|c| {
        column![
            column![
                container(
                    row![
                        text::small("Order #4189")
                            .semibold()
                            .line_height(LineHeight::Absolute(20.into())),
                        space::horizontal(),
                        c.trigger(
                            button(icon::chevrons_up_down().size(16).line_height(1.0))
                                .square()
                                .style(button::Ghost)
                        ),
                    ]
                    .spacing(16)
                    .align_y(Vertical::Center)
                    .width(Fill)
                )
                .padding([0, 16]),
                bordered(
                    row![
                        text::muted("Status").line_height(LineHeight::Absolute(20.into())),
                        space::horizontal(),
                        text::small("Shipped")
                            .medium()
                            .line_height(LineHeight::Absolute(20.into())),
                    ]
                    .width(Fill)
                ),
            ]
            .spacing(8)
            .width(Fill),
            c.content(
                column![
                    detail("Shipping address", "100 Market St, San Francisco"),
                    detail("Items", "2x Studio Headphones"),
                ]
                .spacing(8)
                .width(Fill)
                .padding(Padding::ZERO.top(8))
            ),
        ]
        .width(Fill)
        .into()
    })
    .width(Fill.max(350))
    .into()
}

fn basic<'a, Message: Clone + 'static>() -> Slot<'a, Message> {
    card(
        card::content(
            collapsible(|c| {
                let open = c.is_open();
                let indicator = if open { icon::chevron_up() } else { icon::chevron_down() };
                let surface = theme::container::Transparent + radius(radius::MD);
                let surface = if open { surface + Bg(Token::Muted) } else { surface };
                container(
                    column![
                        c.trigger(
                            button!["Product details", space::horizontal(), indicator.size(16).line_height(1.0)]
                                .width(Fill)
                                .style(button::Ghost)
                        ),
                        c.content(
                            column![
                                text::small("This panel can be expanded or collapsed to reveal additional content.")
                                    .line_height(LineHeight::Absolute(20.into()))
                                    .width(Fill),
                                button("Learn More").size(Size::Xs),
                            ]
                            .width(Fill)
                            .spacing(8)
                            .padding([0, 10, 10, 10])
                        ),
                    ]
                    .width(Fill),
                )
                .width(Fill)
                .style(surface)
                .into()
            })
            .width(Fill),
        )
        .padding([16, 16]),
    ).gap(0).padding(0)
    .width(Fill.max(384))
    .into()
}

fn settings<'a, Message: Clone + 'static>() -> Slot<'a, Message> {
    card(
        column![
            card::header(text::small("Radius").medium().line_height(1.375))
                .inset(12.)
                .description(
                    text::muted("Set the corner radius of the element.")
                        .line_height(LineHeight::Absolute(20.into()))
                ),
            card::content(
                collapsible(|c| {
                    let inputs = || {
                        row![
                            input("0").default_value("0").width(Fill),
                            input("0").default_value("0").width(Fill),
                        ]
                        .spacing(8)
                        .width(Fill)
                    };
                    let indicator = if c.is_open() {
                        icon::minimize()
                    } else {
                        icon::maximize()
                    };
                    row![
                        column![
                            inputs(),
                            c.content(
                                container(inputs())
                                    .width(Fill)
                                    .padding(Padding::ZERO.top(8))
                            ),
                        ]
                        .width(Fill),
                        c.trigger(
                            button(indicator.size(16).line_height(1.0))
                                .square()
                                .style(button::Outline)
                        ),
                    ]
                    .spacing(8)
                    .align_y(Vertical::Top)
                    .width(Fill)
                    .into()
                })
                .width(Fill)
            )
            .padding([0, 12]),
        ]
        .spacing(12)
        .padding([12, 0])
        .width(Fill),
    )
    .gap(0)
    .padding(0)
    .width(Fill.max(320))
    .into()
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

impl page::Page for CollapsiblePage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }
    fn init(&self) -> Task<Message> {
        self.state.init()
    }
    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }
    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: Padding::ZERO,
            ..page::Backdrop::default()
        }
    }
    fn docs_preview(&self, id: &str, outer: &Theme) -> iced::Element<'_, Message, Theme> {
        container(self.themed_preview(id, outer))
            .width(Fill)
            .min_height(if id == "file-tree" { 576 } else { 288 })
            .align_x(Horizontal::Center)
            .align_y(if matches!(id, "demo" | "basic") {
                Vertical::Top
            } else {
                Vertical::Center
            })
            .padding(40)
            .into()
    }
    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => demo().compose(),
            "basic" => basic().compose(),
            "settings" => settings().compose(),
            "file-tree" => file_tree().compose(),
            _ => unreachable!("unknown collapsible preview id: {id}"),
        }
    }
}
