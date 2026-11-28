use iced::{Fill, FillPortion, Task};

use icedcn::widget::field::Orientation;
use icedcn::widget::{button, column, field, input, popover, row, text};
use icedcn::{Size, Theme, View};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Popover",
    description: "Displays rich content in a portal, triggered by a button.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/popover-demo.md",
    },
    page::Block::Md("examples/browser/snippets/popover-usage.md"),
    page::Block::Md("examples/browser/snippets/popover-composition.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some("A simple popover with a header, title, and description."),
        snippet: "examples/browser/snippets/popover-basic.md",
    },
    page::Block::Example {
        id: "alignments",
        title: "Align",
        description: Some("Use `.align(..)` to align the popover to the trigger."),
        snippet: "examples/browser/snippets/popover-alignments.md",
    },
    page::Block::Example {
        id: "form",
        title: "With Form",
        description: Some("A popover with form fields inside."),
        snippet: "examples/browser/snippets/popover-form.md",
    },
    page::Block::Md("examples/browser/snippets/popover-api-reference.md"),
];

/// The popover page. Every popover is uncontrolled; its inputs mount fresh
/// each time it opens.
pub struct PopoverPage {
    state: page::State,
    input_size: f32,
}

impl PopoverPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            input_size: 14.,
        })
    }
}

/// A linked label and an input occupying the remaining two column shares.
fn dimension<'a, M: Clone + 'a>(
    label: impl field::IntoLabel<'a, M>,
    value: &'a str,
    size: f32,
) -> View<'a, M> {
    field(
        input("")
            .default_value(value)
            .text_size(size)
            .width(FillPortion(2)),
    )
    .label(label)
    .orientation(Orientation::Horizontal)
    .control_last()
    .width(Fill)
    .into()
}

/// An outline `size="sm"` trigger whose `w-40` panel is aligned `align`.
fn aligned<'a>(label: &'a str, align: popover::Align, content: &'a str) -> View<'a, Message> {
    popover(
        move |p| {
            p.trigger(
                button(label)
                    .size(Size::Sm)
                    .style(button::Outline)
                    .has_popup(true),
            )
        },
        move |_| text(content).into(),
    )
    .align(align)
    .width(160)
    .into()
}

impl page::Page for PopoverPage {
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
        self.input_size = if width < 768. { 16. } else { 14. };
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let size = self.input_size;
        match id {
            "demo" => popover(
                |p| {
                    p.trigger(
                        button("Open popover")
                            .style(button::Outline)
                            .has_popup(true),
                    )
                },
                move |_| {
                    column![
                        column![
                            text::label("Dimensions").height(14),
                            text::muted("Set the dimensions for the layer."),
                        ]
                        .spacing(8),
                        field::group([
                            dimension(text::label("Width"), "100%", size),
                            dimension(text::label("Max. width"), "300px", size),
                            dimension(text::label("Height"), "25px", size),
                            dimension(text::label("Max. height"), "none", size),
                        ])
                        .spacing(8),
                    ]
                    .spacing(16)
                    .width(Fill)
                    .into()
                },
            )
            .width(320)
            .into(),

            "basic" => popover(
                |p| {
                    p.trigger(
                        button("Open Popover")
                            .style(button::Outline)
                            .has_popup(true),
                    )
                },
                |_| popover::header("Dimensions", "Set the dimensions for the layer."),
            )
            .align(popover::Align::Start)
            .into(),

            "alignments" => row![
                aligned("Start", popover::Align::Start, "Aligned to start"),
                aligned("Center", popover::Align::Center, "Aligned to center"),
                aligned("End", popover::Align::End, "Aligned to end"),
            ]
            .spacing(24)
            .into(),

            "form" => popover(
                |p| {
                    p.trigger(
                        button("Open Popover")
                            .style(button::Outline)
                            .has_popup(true),
                    )
                },
                move |_| {
                    column![
                        popover::header("Dimensions", "Set the dimensions for the layer."),
                        field::group([
                            dimension("Width", "100%", size),
                            dimension("Height", "25px", size),
                        ])
                        .spacing(16)
                    ]
                    .width(Fill)
                    .spacing(10)
                    .into()
                },
            )
            .align(popover::Align::Start)
            .width(256)
            .into(),

            _ => unreachable!("unknown popover preview id: {id}"),
        }
    }
}
