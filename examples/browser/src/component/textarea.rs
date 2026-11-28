use iced::{Fill, Task};

use icedcn::theme;
use icedcn::theme::tweak::Alpha;
use icedcn::widget::text;
use icedcn::widget::{button, column, container, field, textarea};
use icedcn::{Theme, View};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Textarea",
    description: "Displays a form textarea or a component that looks like a textarea.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/textarea-demo.md",
    },
    page::Block::Md("examples/browser/snippets/textarea-usage.md"),
    page::Block::Example {
        id: "field",
        title: "Field",
        description: Some("Use `field(..)` to add a label and a description."),
        snippet: "examples/browser/snippets/textarea-field.md",
    },
    page::Block::Example {
        id: "disabled",
        title: "Disabled",
        description: Some("Use `.disabled(true)` to disable the textarea."),
        snippet: "examples/browser/snippets/textarea-disabled.md",
    },
    page::Block::Example {
        id: "invalid",
        title: "Invalid",
        description: Some("Use `.style(textarea::Invalid)` to show an error state."),
        snippet: "examples/browser/snippets/textarea-invalid.md",
    },
    page::Block::Example {
        id: "button",
        title: "Button",
        description: Some("Pair the textarea with a button."),
        snippet: "examples/browser/snippets/textarea-button.md",
    },
];

pub struct TextareaPage {
    state: page::State,
    text_size: f32,
}

impl TextareaPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            text_size: 14.,
        })
    }
}

/// The preview's `w-full` column, capped like the docs' preview.
fn full<'a>(content: impl Into<View<'a, Message>>) -> iced::Element<'a, Message, Theme> {
    container(content).width(Fill).max_width(320).into()
}

impl page::Page for TextareaPage {
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
        self.text_size = if width < 768. { 16. } else { 14. };
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 0.into(),
            ..Default::default()
        }
    }

    fn docs_preview(&self, id: &str, theme: &Theme) -> iced::Element<'_, Message, Theme> {
        container(self.themed_preview(id, theme))
            .center_x(Fill)
            .center_y(288)
            .padding([0, 40])
            .into()
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let area = || textarea("Type your message here.").text_size(self.text_size);
        match id {
            "demo" => full(area()),

            "field" => full(
                field(area())
                    .label("Message")
                    .description("Enter your message below.")
                    .description_above(),
            ),

            "disabled" => full(
                field(area())
                    .disabled(true)
                    .label(text::label("Message").style(theme::text::Foreground + Alpha(0.5))),
            ),

            "invalid" => full(
                field(area().style(textarea::Invalid))
                    .label(text::label("Message").style(theme::text::Destructive))
                    .description("Please enter a valid message."),
            ),

            "button" => full(
                column![area(), button("Send message").width(Fill),]
                    .spacing(8)
                    .width(Fill),
            ),

            _ => unreachable!("unknown textarea preview id: {id}"),
        }
    }
}
