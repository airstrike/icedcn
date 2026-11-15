mod controls;
mod demo;
mod fixtures;
mod frame;
mod skin;
mod turn;

use self::demo::Demo;
use crate::page::rhea;
use crate::{Message, page};
use iced::widget::component::component;
use iced::{Task, Widget};
use icedcn::Theme;

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
    page::Block::Md("examples/browser/snippets/message-scroller-installation.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-usage.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-composition.md"),
    page::Block::Group {
        title: "Core Concepts",
        children: &[
            page::Block::Example {
                id: "anchoring",
                title: "Anchoring Turns",
                description: Some(
                    "Mark the row that starts an exchange with `.anchor(true)`. Choose User or Assistant and send a few messages to see which row the viewport holds in place.",
                ),
                snippet: "examples/browser/snippets/message-scroller-anchoring.md",
            },
            page::Block::Example {
                id: "group-chat",
                title: "Group Chat",
                description: Some(
                    "A turn can start with an event as well as a message. Add Rocky to anchor the join marker, then send a reply as Rocky.",
                ),
                snippet: "examples/browser/snippets/message-scroller-group-chat.md",
            },
            page::Block::Example {
                id: "previous-context",
                title: "Keeping Context Visible",
                description: Some(
                    "Use `.previous_item_peek(...)` to leave part of the preceding reply above a new turn. Adjust the slider before sending to change how much context stays visible.",
                ),
                snippet: "examples/browser/snippets/message-scroller-previous-context.md",
            },
            page::Block::Example {
                id: "streaming",
                title: "Following the Live Edge",
                description: Some(
                    "With `.auto_scroll(true)`, a growing reply follows the reader while they are at the bottom. Scroll up during a reply to read earlier messages, then use the down arrow to return.",
                ),
                snippet: "examples/browser/snippets/message-scroller-streaming.md",
            },
            page::Block::Example {
                id: "opening-position",
                title: "Opening Saved Threads",
                description: Some(
                    "Use `.default_scroll_position(...)` when mounting an existing thread. Compare its beginning, its most recent output, and the last anchored turn.",
                ),
                snippet: "examples/browser/snippets/message-scroller-opening-position.md",
            },
            page::Block::Md("examples/browser/snippets/message-scroller-initial-layout.md"),
            page::Block::Example {
                id: "load-history",
                title: "Loading Earlier Messages",
                description: Some(
                    "Prepend older messages with stable keys. The reader should see the same part of the conversation before and after the history arrives.",
                ),
                snippet: "examples/browser/snippets/message-scroller-load-history.md",
            },
            page::Block::Example {
                id: "animation",
                title: "Animating New Messages",
                description: Some(
                    "Animate the newly mounted user row. Its final dimensions are available to the scroller throughout the entrance, so the assistant can stream into place beneath it.",
                ),
                snippet: "examples/browser/snippets/message-scroller-animation.md",
            },
            page::Block::Example {
                id: "commands",
                title: "Jumping to Messages",
                description: Some(
                    "Keep a stable key for each transcript row and pass it in `Target::Item`. The menu uses those keys to jump directly to an exchange.",
                ),
                snippet: "examples/browser/snippets/message-scroller-commands.md",
            },
            page::Block::Example {
                id: "visibility",
                title: "Tracking the Reader's Position",
                description: Some(
                    "With `.on_reader_change(...)`, the scroller publishes a message containing the current anchor and the keys of visible rows. Hover over the outline, or open Outline on a narrow screen, to find the current turn and jump to another.",
                ),
                snippet: "examples/browser/snippets/message-scroller-visibility.md",
            },
            page::Block::Example {
                id: "scrollable",
                title: "Reading Scroll State",
                description: Some(
                    "Read `can_scroll_up` and `can_scroll_down` from the `scroller::Reader` in your message to find which directions have more content. Scroll the conversation and watch the footer change.",
                ),
                snippet: "examples/browser/snippets/message-scroller-scrollable.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/message-scroller-performance.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-virtualization.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-accessibility.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-unstyled.md"),
    page::Block::Md("examples/browser/snippets/message-scroller-api-reference.md"),
];
pub struct Page {
    state: page::State,
    width: f32,
}

impl Page {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            width: 1440.,
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

    fn window_width(&mut self, width: f32) {
        self.width = width;
    }

    fn backdrop(&self) -> page::Backdrop {
        let mut backdrop = rhea::backdrop();
        backdrop.padding = match self.width {
            width if width < 480. => 16.into(),
            width if width < 560. => 32.into(),
            width if width < 640. => 40.into(),
            _ => backdrop.padding,
        };
        backdrop
    }

    fn preview_theme(&self, outer: &Theme) -> Option<Theme> {
        Some(rhea::theme(outer))
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
            "visibility" => Demo::Visibility {
                compact: self.width < 640.,
            },
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
