use iced::Task;

use icedcn::widget::{button, row, toast};
use icedcn::{Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Toast",
    description: "Display a brief notification when an action succeeds, fails, or needs attention.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/toast-demo.md",
    },
    page::Block::Md("examples/browser/snippets/toast-guide-about.md"),
    page::Block::Md("examples/browser/snippets/toast-installation.md"),
    page::Block::Md("examples/browser/snippets/toast-usage.md"),
    page::Block::Example {
        id: "types",
        title: "Types",
        description: Some("Use `.kind(..)` to pick the toast's icon."),
        snippet: "examples/browser/snippets/toast-types.md",
    },
    page::Block::Example {
        id: "description",
        title: "Description",
        description: Some("Use `.description(..)` to add a description to the toast."),
        snippet: "examples/browser/snippets/toast-description.md",
    },
    page::Block::Example {
        id: "position",
        title: "Position",
        description: Some("Use `.position(..)` to change where the toast appears."),
        snippet: "examples/browser/snippets/toast-position.md",
    },
    page::Block::Md("examples/browser/snippets/toast-api-reference.md"),
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

/// An outline button raising `toast`.
fn raise<'a>(label: &'a str, toast: toast::Toast<Message>) -> Slot<'a, Message> {
    toast::trigger(button(label).style(button::Outline), toast)
}

/// "Event has been created" toast.
fn created() -> toast::Toast<Message> {
    toast::toast("Event has been created")
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
        match id {
            "demo" => raise(
                "Show Toast",
                created()
                    .description("Sunday, December 03, 2023 at 9:00 AM")
                    .action(
                        "Undo",
                        Message::Page(self.state.page_index, page::Message::Noop),
                    ),
            )
            .into(),

            "types" => row![
                raise("Default", created()),
                raise("Success", created().kind(toast::Kind::Success)),
                raise(
                    "Info",
                    toast::toast("Be at the area 10 minutes before the event time")
                        .kind(toast::Kind::Info),
                ),
                raise(
                    "Warning",
                    toast::toast("Event start time cannot be earlier than 8am")
                        .kind(toast::Kind::Warning),
                ),
                raise(
                    "Error",
                    toast::toast("Event has not been created").kind(toast::Kind::Error),
                ),
                button("Promise")
                    .style(button::Outline)
                    .on_press(Message::CreateEvent),
            ]
            .spacing(8)
            .wrap()
            .into(),

            "description" => raise(
                "Show Toast",
                created().description("Monday, January 3rd at 6:00pm"),
            )
            .into(),

            "position" => {
                use toast::Position::*;
                let controls = [
                    ("Top Left", TopLeft),
                    ("Top Center", TopCenter),
                    ("Top Right", TopRight),
                    ("Bottom Left", BottomLeft),
                    ("Bottom Center", BottomCenter),
                    ("Bottom Right", BottomRight),
                ];
                row::Row::with_children(
                    controls.map(|(label, place)| raise(label, created().position(place))),
                )
                .spacing(8)
                .wrap()
                .align_x(iced::Center)
                .into()
            }

            _ => unreachable!("unknown toast preview id: {id}"),
        }
    }
}
