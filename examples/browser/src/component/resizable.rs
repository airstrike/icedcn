use crate::{Message, page};
use iced::Fill;
use iced::Task;
use icedcn::Theme;
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::{container, resizable, text};
use icedcn::{Orientation, Slot};

pub static META: page::Meta = page::Meta {
    title: "Resizable",
    description: "Accessible resizable panel groups and layouts with keyboard support.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/resizable-demo.md",
    },
    page::Block::Md("examples/browser/snippets/resizable-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "vertical",
                title: "Vertical",
                description: Some("Use the vertical orientation to stack the panels."),
                snippet: "examples/browser/snippets/resizable-vertical.md",
            },
            page::Block::Example {
                id: "handle",
                title: "Handle",
                description: Some("Use a visible handle to mark the divider."),
                snippet: "examples/browser/snippets/resizable-handle.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/resizable-api-reference.md"),
];
fn panel(label: &'static str) -> Slot<'static, Message> {
    container(text::p(label).semibold()).center(Fill).into()
}
fn frame(group: impl Into<Slot<'static, Message>>, width: f32) -> Slot<'static, Message> {
    container(group)
        .width(Fill)
        .max_width(width)
        .height(200)
        .padding(1)
        .clip(true)
        .style(|theme| Style {
            border: Border {
                width: 1.0,
                color: theme.neutral.border,
                radius: 8.into(),
            },
            ..Style::default()
        })
        .into()
}
pub struct ResizablePage {
    state: page::State,
}

impl ResizablePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for ResizablePage {
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
            "demo" => frame(
                resizable(
                    panel("One"),
                    resizable(panel("Two"), panel("Three"))
                        .orientation(Orientation::Vertical)
                        .default_value(0.25)
                        .with_handle(),
                )
                .with_handle(),
                384.0,
            )
            .into(),
            "vertical" => frame(
                resizable(panel("Header"), panel("Content"))
                    .orientation(Orientation::Vertical)
                    .default_value(0.25),
                384.0,
            )
            .into(),
            "handle" => frame(
                resizable(panel("Sidebar"), panel("Content"))
                    .default_value(0.25)
                    .with_handle(),
                450.0,
            )
            .into(),
            _ => unreachable!("unknown resizable preview id: {id}"),
        }
    }
}
