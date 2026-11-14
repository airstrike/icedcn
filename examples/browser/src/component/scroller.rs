mod controls;
mod demo;
mod fixtures;
mod frame;
mod tick;
mod turn;

use self::demo::Demo;
use crate::{Message, page};
use iced::widget::component::component;
use iced::{Task, Widget};
use icedxdy::Theme;

pub static META: page::Meta = page::Meta {
    title: "Message Scroller",
    description: "A chat scroll container that anchors turns, opens saved transcripts, follows streamed responses, loads history without jumping, and jumps to any message.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/message-scroller-demo.md",
    },
    page::Block::Md("examples/browser/snippets/message-scroller-experience.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-overview.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-composition.md"),
    page::Block::Group {
        title: "Core Concepts",
        children: &[
            page::Block::Example {
                id: "anchoring",
                title: "Anchoring Turns",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-anchoring.md",
            },
            page::Block::Example {
                id: "group-chat",
                title: "Group Chat",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-group-chat.md",
            },
            page::Block::Example {
                id: "previous-context",
                title: "Keeping Context Visible",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-previous-context.md",
            },
            page::Block::Example {
                id: "streaming",
                title: "Following the Live Edge",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-streaming.md",
            },
            page::Block::Example {
                id: "opening-position",
                title: "Opening Saved Threads",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-opening-position.md",
            },
            page::Block::Md("examples/browser/snippets/message-scroller-initial-layout.md"),
            page::Block::Example {
                id: "load-history",
                title: "Loading Earlier Messages",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-load-history.md",
            },
            page::Block::Example {
                id: "animation",
                title: "Animating New Messages",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-animation.md",
            },
            page::Block::Example {
                id: "commands",
                title: "Jumping to Messages",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-commands.md",
            },
            page::Block::Example {
                id: "visibility",
                title: "Tracking the Reader's Position",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-visibility.md",
            },
            page::Block::Example {
                id: "scrollable",
                title: "Reading Scroll State",
                description: None,
                snippet: "examples/browser/snippets/message-scroller-scrollable.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/message-scroller-performance.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-virtualization.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-accessibility.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-api-reference.md"),
];
pub struct Page {
    state: page::State,
}

impl Page {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for Page {
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

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let mode = match id {
            "demo" => Demo::Chat,
            "anchoring" => Demo::Anchoring,
            "group-chat" => Demo::Group,
            "previous-context" => Demo::Context,
            "streaming" => Demo::Streaming,
            "opening-position" => Demo::Opening,
            "load-history" => Demo::History,
            "animation" => Demo::Animation,
            "commands" => Demo::Commands,
            "visibility" => Demo::Visibility,
            "scrollable" => Demo::Scrollable,
            _ => unreachable!("unknown message-scroller preview id: {id}"),
        };
        let index = self.page_index();
        component(mode)
            .boxed()
            .map(move |event| Message::Page(index, event))
            .boxed()
    }
}
