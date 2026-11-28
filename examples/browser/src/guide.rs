//! Native documentation guides using the shared page renderer and section navigation.
use iced::widget::Space;
use iced::{Element, Task, Widget};
use icedcn::theme::{Bg, Fg, hover, radius};
use icedcn::widget::{badge, button, column, row};
use icedcn::{Destructive, Theme, View, card};

use crate::{Message, page};

struct Article {
    meta: page::Meta,
    blocks: &'static [page::Block],
}

static ARTICLES: &[Article] = &[
    Article {
        meta: page::Meta {
            title: "Introduction",
            description: "A practical starting point for building native interfaces with icedcn.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-introduction-start.md"),
            page::Block::Md("examples/browser/snippets/guide-introduction-model.md"),
        ],
    },
    Article {
        meta: page::Meta {
            title: "Components",
            description: "Find the controls, layouts, and interaction patterns for your application.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-components-choose.md"),
            page::Block::Md("examples/browser/snippets/guide-components-compose.md"),
            page::Block::Md("examples/browser/snippets/guide-components-state.md"),
        ],
    },
    Article {
        meta: page::Meta {
            title: "Conversations",
            description: "Build a transcript with stable messages, streaming updates, and deliberate scrolling.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-conversations-transcript.md"),
            page::Block::Md("examples/browser/snippets/guide-conversations-streaming.md"),
            page::Block::Md("examples/browser/snippets/guide-conversations-controls.md"),
        ],
    },
    Article {
        meta: page::Meta {
            title: "Installation",
            description: "Add icedcn to a Rust application, open your first window, and explore the examples.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-installation-dependency.md"),
            page::Block::Md("examples/browser/snippets/guide-installation-window.md"),
            page::Block::Md("examples/browser/snippets/guide-installation-icons.md"),
            page::Block::Md("examples/browser/snippets/guide-installation-browser.md"),
        ],
    },
    Article {
        meta: page::Meta {
            title: "Theming",
            description: "Apply light and dark palettes, style widgets with variants and tweaks, and build your own design.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-theming-mode.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-roles.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-styles.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-variants.md"),
            page::Block::Example {
                id: "theming-variants",
                title: "",
                description: None,
                snippet: "examples/browser/snippets/guide-theming-variants-demo.md",
            },
            page::Block::Md("examples/browser/snippets/guide-theming-variants-shared.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-tweaks.md"),
            page::Block::Example {
                id: "theming-tweaks",
                title: "",
                description: None,
                snippet: "examples/browser/snippets/guide-theming-tweaks-demo.md",
            },
            page::Block::Md("examples/browser/snippets/guide-theming-tweaks-list.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-custom.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-components.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-design.md"),
        ],
    },
    Article {
        meta: page::Meta {
            title: "Cargo",
            description: "Understand the workspace, optional features, and development commands.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-cargo-workspace.md"),
            page::Block::Md("examples/browser/snippets/guide-cargo-features.md"),
            page::Block::Md("examples/browser/snippets/guide-cargo-checks.md"),
        ],
    },
    Article {
        meta: page::Meta {
            title: "Changelog",
            description: "Trace implemented changes to their source commits.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-changelog-changes.md"),
            page::Block::Md("examples/browser/snippets/guide-changelog-updating.md"),
        ],
    },
];

struct Guide {
    article: &'static Article,
    state: page::State,
}

/// Create the documentation pages after the component registry.
pub fn pages(start: usize) -> impl Iterator<Item = Box<dyn page::Page>> {
    ARTICLES
        .iter()
        .enumerate()
        .map(move |(offset, article)| -> Box<dyn page::Page> {
            Box::new(Guide {
                article,
                state: page::State::new(start + offset, article.blocks),
            })
        })
}

impl page::Page for Guide {
    fn meta(&self) -> &'static page::Meta {
        &self.article.meta
    }
    fn blocks(&self) -> &'static [page::Block] {
        self.article.blocks
    }
    fn page_index(&self) -> usize {
        self.state.page_index
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
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        let index = self.state.page_index;
        match id {
            "theming-variants" => deployment()
                .map(move |()| Message::Page(index, page::Message::Noop))
                .into(),
            "theming-tweaks" => tweaked()
                .map(move |()| Message::Page(index, page::Message::Noop))
                .into(),
            _ => Space::new().boxed(),
        }
    }
}

fn deployment<'a>() -> View<'a> {
    card(column![
        card::header("api-gateway")
            .description("Build stopped at step 3 of 5.")
            .action(badge("Failed").style(badge::Destructive)),
        card::content(
            row![
                badge("main").style(badge::Outline),
                badge("v2.4.1").style(badge::Secondary),
            ]
            .spacing(8)
        ),
        card::footer(
            row![
                button("Retry"),
                button("View logs").style(button::Outline),
                button("Dismiss").style(icedcn::Ghost),
            ]
            .spacing(8)
        ),
    ])
    .width(360)
    .into()
}

fn tweaked<'a>() -> View<'a> {
    let quiet = button::Ghost + Fg::MutedForeground + hover(Fg::Foreground);
    row![
        button("Muted").style(Destructive + Bg::Muted),
        button("Square").style(button::Outline + radius(0)),
        button("Quiet").style(quiet),
    ]
    .spacing(8)
    .into()
}
