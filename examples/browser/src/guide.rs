//! Native documentation guides using the shared page renderer and section navigation.
use iced::widget::Space;
use iced::{Element, Task, Widget};
use icedcn::Theme;

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
            description: "Run the browser and connect an application to the same Rust dependencies.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-installation-browser.md"),
            page::Block::Md("examples/browser/snippets/guide-installation-dependency.md"),
            page::Block::Md("examples/browser/snippets/guide-installation-window.md"),
            page::Block::Md("examples/browser/snippets/guide-installation-icons.md"),
        ],
    },
    Article {
        meta: page::Meta {
            title: "Theming",
            description: "Apply light and dark palettes, semantic roles, and widget styles.",
        },
        blocks: &[
            page::Block::Md("examples/browser/snippets/guide-theming-mode.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-roles.md"),
            page::Block::Md("examples/browser/snippets/guide-theming-styles.md"),
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
    fn preview(&self, _: &str) -> Element<'_, Message, Theme> {
        Space::new().boxed()
    }
}
