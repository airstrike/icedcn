// API Reference is rendered via the `dialog-api-reference.md` snippet at the
// bottom of BLOCKS.
//
// Intentional skips:
// - dialog-rtl — RTL is not in scope (per CLAUDE.md).

use iced::widget::responsive;
use iced::{Shrink, Task, Widget};

use icedcn::widget::{button, column, dialog, field, input, scrollable, text};
use icedcn::{Slot, Theme};

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
    page::Block::Md("examples/browser/snippets/dialog-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "close-button",
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
        ],
    },
    page::Block::Md("examples/browser/snippets/dialog-api-reference.md"),
];

/// The dialog page. Every dialog is uncontrolled; its inputs keep what is
/// typed while it stays open, and mount fresh each time it opens.
pub struct DialogPage {
    state: page::State,
}

impl DialogPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn labelled<'a>(label: &'a str, value: &'a str) -> Slot<'a, Message> {
    field(input("").default_value(value).width(iced::Fill))
        .label(label)
        .into()
}

const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn scrolling(
    title: &'static str,
    description: &'static str,
    footer: bool,
) -> iced::Element<'static, Message, Theme> {
    dialog(
        move |d| d.trigger(button(title).style(button::Outline)),
        move |d| {
            let content = responsive(|size| {
                let paragraphs = column((0..10).map(|_| text(PARAGRAPH).into())).spacing(16);
                Slot::from(scrollable(paragraphs).height((size.height + 64.0) / 2.0)).compose()
            })
            .height(Shrink)
            .boxed();
            let mut body = column![dialog::header(title, description), content].spacing(16);
            if footer {
                body = body.push(dialog::footer([
                    d.close(button("Close").style(button::Outline))
                ]));
            }
            body.into()
        },
    )
    .into()
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
                |d| {
                    column![
                        dialog::header(
                            "Edit profile",
                            "Make changes to your profile here. Click save when you're done.",
                        ),
                        column![
                            labelled("Name", "Pedro Duarte"),
                            labelled("Username", "@peduarte"),
                        ]
                        .spacing(16),
                        dialog::footer([
                            d.close(button("Cancel").style(button::Outline)),
                            d.close(button("Save changes")),
                        ]),
                    ]
                    .spacing(16)
                    .into()
                },
            )
            .into(),

            "close-button" => dialog(
                |d| d.trigger(button("Share").style(button::Outline)),
                |d| {
                    column![
                        dialog::header(
                            "Share link",
                            "Anyone who has this link will be able to view this.",
                        ),
                        input("")
                            .default_value("https://github.com/airstrike/icedcn")
                            .width(iced::Fill),
                        dialog::footer([d.close(button("Close"))]),
                    ]
                    .spacing(16)
                    .into()
                },
            )
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
