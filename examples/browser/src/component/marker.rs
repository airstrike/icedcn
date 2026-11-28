use crate::page::theme_blue;
use crate::{Message, icon, page};
use iced::widget::Id;
use iced::{Element, Fill, Task};
use icedcn::widget::status_marker::{Border, Separator, shimmer};
use icedcn::widget::{button, column, container, spinner, status_marker, toast};
use icedcn::{Theme, View};
use page::Block;

pub static META: page::Meta = page::Meta {
    title: "Marker",
    description: "Displays an inline status, system note, bordered row, or labeled separator in a conversation.",
};
pub static BLOCKS: &[Block] = &[
    Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/marker-demo.md",
    },
    Block::Md("examples/browser/snippets/marker-composition.md"),
    Block::Example {
        id: "variants",
        title: "Variants",
        description: None,
        snippet: "examples/browser/snippets/marker-variants.md",
    },
    Block::Example {
        id: "status",
        title: "Status",
        description: None,
        snippet: "examples/browser/snippets/marker-status.md",
    },
    Block::Example {
        id: "shimmer",
        title: "Shimmer",
        description: None,
        snippet: "examples/browser/snippets/marker-shimmer.md",
    },
    Block::Example {
        id: "separator",
        title: "Separator",
        description: None,
        snippet: "examples/browser/snippets/marker-separator.md",
    },
    Block::Example {
        id: "border",
        title: "Border",
        description: None,
        snippet: "examples/browser/snippets/marker-border.md",
    },
    Block::Example {
        id: "icon",
        title: "With Icon",
        description: None,
        snippet: "examples/browser/snippets/marker-icon.md",
    },
    Block::Example {
        id: "link-button",
        title: "Links and Buttons",
        description: None,
        snippet: "examples/browser/snippets/marker-link-button.md",
    },
    Block::Md("examples/browser/snippets/marker-api-reference.md"),
];
pub struct MarkerPage {
    state: page::State,
}
impl MarkerPage {
    pub fn new(index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(index, BLOCKS),
        })
    }
}
fn view(id: &str, page_index: usize) -> View<'static, Message> {
    let group = match id {
        "demo" => column![
            status_marker("Switched to a new branch").icon(icon::git_branch()),
            status_marker(shimmer("Thinking...")).icon(spinner().size(16.)),
            status_marker("Conversation compacted").style(Separator),
            status_marker("Explored 4 files").icon(icon::search()),
        ]
        .spacing(32),
        "variants" => column![
            status_marker("A default marker for inline notes."),
            status_marker("A separator marker").style(Separator),
            status_marker("A border marker for row boundaries.").style(Border),
        ]
        .spacing(32),
        "status" => column![
            status_marker("Compacting conversation").icon(spinner().size(16.)),
            status_marker("Running tests")
                .icon(spinner().size(16.))
                .style(Separator),
        ]
        .spacing(32),
        "shimmer" => column![
            status_marker(shimmer("Thinking...")),
            status_marker(shimmer("Reading 4 files")).style(Separator),
        ]
        .spacing(32),
        "separator" => column![
            status_marker("Today").style(Separator),
            status_marker("Worked for 42s").style(Separator),
            status_marker("Conversation compacted").style(Separator),
        ]
        .spacing(32),
        "border" => column![
            status_marker("Switched to release-candidate")
                .icon(icon::git_branch())
                .style(Border),
            status_marker("Reviewed 8 related files")
                .icon(icon::search())
                .style(Border),
            status_marker("Opened implementation notes")
                .icon(icon::file_text())
                .style(Border),
        ]
        .spacing(12),
        "icon" => column![
            status_marker("Switched to a new branch").icon(icon::git_branch()),
            status_marker("Explored 4 files")
                .icon(icon::search())
                .style(Separator),
            status_marker("Syncing completed")
                .icon(icon::book_open_check())
                .stacked(true),
        ]
        .spacing(48),
        "link-button" => column![
            button(status_marker("View the pull request").icon(icon::git_branch()))
                .style(button::Link)
                .inset(0)
                .on_press(Message::Page(
                    page_index,
                    page::Message::LinkClicked("#links-and-buttons".into())
                )),
            toast::trigger(
                button(status_marker("Revert this change").icon(icon::refresh_ccw()))
                    .style(button::Ghost)
                    .inset(0),
                toast::toast("You clicked the revert button")
            ),
        ]
        .spacing(32),
        _ => unreachable!("unknown Marker example {id}"),
    };
    container(group.width(Fill).padding([48, 0]))
        .width(Fill)
        .max_width(384)
        .into()
}
impl page::Page for MarkerPage {
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
    fn scrollable_id(&self) -> Id {
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
            min_height: 0.0,
            padding: 40.into(),
            ..page::Backdrop::default()
        }
    }
    fn preview_theme(&self, outer: &Theme) -> Option<Theme> {
        Some(theme_blue::theme(outer))
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        view(id, self.state.page_index).into()
    }
}
