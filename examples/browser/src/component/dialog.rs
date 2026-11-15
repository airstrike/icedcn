// RTL is outside the supported examples.
use iced::alignment::Horizontal;
use iced::{Fill, Task};

use icedcn::Theme;
use icedcn::widget::{button, column, dialog, field, input, text};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Dialog",
    description: "A window overlaid on either the primary window or another dialog window, \
                  rendering the content underneath inert.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/dialog-demo.md",
    },
    page::Block::Md("examples/browser/snippets/dialog-usage.md"),
    page::Block::Md("examples/browser/snippets/dialog-composition.md"),
    page::Block::Example {
        id: "custom-close-button",
        title: "Custom Close Button",
        description: Some("Replace the default close control with your own button."),
        snippet: "examples/browser/snippets/dialog-close-button.md",
    },
    page::Block::Example {
        id: "no-close-button",
        title: "No Close Button",
        description: Some("Use `.show_close_button(false)` to hide the close button."),
        snippet: "examples/browser/snippets/dialog-no-close-button.md",
    },
    page::Block::Example {
        id: "sticky-footer",
        title: "Sticky Footer",
        description: Some("Keep actions visible while the content scrolls."),
        snippet: "examples/browser/snippets/dialog-sticky-footer.md",
    },
    page::Block::Example {
        id: "scrollable-content",
        title: "Scrollable Content",
        description: Some("Long content can scroll while the header stays in view."),
        snippet: "examples/browser/snippets/dialog-scrollable-content.md",
    },
    page::Block::Md("examples/browser/snippets/dialog-api-reference.md"),
];

/// The dialog page. Every dialog is uncontrolled; its inputs keep what is
/// typed while it stays open, and mount fresh each time it opens.
pub struct DialogPage {
    state: page::State,
    input_size: f32,
}

impl DialogPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            input_size: 14.,
        })
    }
}

const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn scrolling(
    title: &'static str,
    description: &'static str,
    footer: bool,
) -> iced::Element<'static, Message, Theme> {
    let recipe = dialog(
        move |d| d.trigger(button(title).style(button::Outline)),
        move |_| dialog::header(title, description),
    )
    .scrollable_body(|_| {
        column((0..10).map(|_| text::small(PARAGRAPH).line_height(1.5).into()))
            .spacing(16)
            .into()
    });
    if footer {
        recipe
            .footer(|d| [d.close(button("Close").style(button::Outline))])
            .into()
    } else {
        recipe.into()
    }
}

impl page::Page for DialogPage {
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

    fn window_width(&mut self, width: f32) {
        self.input_size = if width < 768. { 16. } else { 14. };
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
            "demo" => dialog(
                |d| d.trigger(button("Open Dialog").style(button::Outline)),
                |_| {
                    let profile_field = |label, value| {
                        field(
                            input("")
                                .default_value(value)
                                .text_size(self.input_size)
                                .width(Fill),
                        )
                        .label(text::label(label).height(14))
                        .width(Fill)
                        .into()
                    };
                    column![
                        dialog::header(
                            "Edit profile",
                            "Make changes to your profile here. Click save when you're done.",
                        ),
                        field::group([
                            profile_field("Name", "Pedro Duarte"),
                            profile_field("Username", "@peduarte"),
                        ]),
                    ]
                    .spacing(16)
                    .width(Fill)
                    .into()
                },
            )
            .footer(|d| {
                [
                    d.close(button("Cancel").style(button::Outline)),
                    button("Save changes").into(),
                ]
            })
            .into(),
            "custom-close-button" => dialog(
                |d| d.trigger(button("Share").style(button::Outline)),
                |_| {
                    column![
                        dialog::header(
                            "Share link",
                            "Anyone who has this link will be able to view this."
                        ),
                        input("")
                            .value("https://github.com/airstrike/icedcn")
                            .text_size(self.input_size)
                            .width(Fill),
                    ]
                    .spacing(16)
                    .width(Fill)
                    .into()
                },
            )
            .max_width(448.)
            .footer(|d| [d.close(button("Close"))])
            .footer_align_x(Horizontal::Left)
            .into(),
            "no-close-button" => dialog(
                |d| d.trigger(button("No Close Button").style(button::Outline)),
                |_| {
                    dialog::header(
                        "No Close Button",
                        "This dialog doesn't have a close button in the top-right corner.",
                    )
                },
            )
            .show_close_button(false)
            .into(),

            "sticky-footer" => scrolling(
                "Sticky Footer",
                "This dialog has a sticky footer that stays visible while the content scrolls.",
                true,
            ),
            "scrollable-content" => scrolling(
                "Scrollable Content",
                "This is a dialog with scrollable content.",
                false,
            ),
            _ => unreachable!("unknown dialog preview id: {id}"),
        }
    }
}
