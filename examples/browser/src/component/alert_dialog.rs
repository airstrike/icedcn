// The RTL example is outside the project’s scope.

use iced::widget::text::LineHeight;
use iced::widget::{rich_text, span, svg};
use iced::{Fill, Task, Widget};
use icedxdy::theme::text::muted;

use icedxdy::widget::{alert_dialog, button, column};
use icedxdy::{Size, Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Alert Dialog",
    description: "A modal dialog that interrupts the user with important content and expects a response.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/alert-dialog-demo.md",
    },
    page::Block::Md("examples/browser/snippets/alert-dialog-installation.md"),
    page::Block::Md("examples/browser/snippets/alert-dialog-usage.md"),
    page::Block::Md("examples/browser/snippets/alert-dialog-composition.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some(
            "A basic alert dialog with a title, description, and cancel and continue buttons.",
        ),
        snippet: "examples/browser/snippets/alert-dialog-basic.md",
    },
    page::Block::Example {
        id: "small",
        title: "Small",
        description: Some("Use the `.size(Size::Sm)` builder to make the alert dialog smaller."),
        snippet: "examples/browser/snippets/alert-dialog-small.md",
    },
    page::Block::Example {
        id: "media",
        title: "Media",
        description: Some(
            "Use the `.media(..)` header builder to add a media element such as an icon or image to the alert dialog.",
        ),
        snippet: "examples/browser/snippets/alert-dialog-media.md",
    },
    page::Block::Example {
        id: "small-media",
        title: "Small with Media",
        description: Some(
            "Use the `.size(Size::Sm)` builder to make the alert dialog smaller and the `.media(..)` header builder to add a media element such as an icon or image to the alert dialog.",
        ),
        snippet: "examples/browser/snippets/alert-dialog-small-media.md",
    },
    page::Block::Example {
        id: "destructive",
        title: "Destructive",
        description: Some(
            "Use the `.action(..)` handle method to add a destructive action button to the alert dialog.",
        ),
        snippet: "examples/browser/snippets/alert-dialog-destructive.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[page::Block::Md(
            "examples/browser/snippets/alert-dialog-api-reference.md",
        )],
    },
];

pub struct AlertDialogPage {
    state: page::State,
}

impl AlertDialogPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// "Show Dialog" outline trigger.
fn show(a: &alert_dialog::Handle) -> Slot<'static, Message> {
    a.trigger(button("Show Dialog").style(button::Outline))
}

impl page::Page for AlertDialogPage {
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

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            min_height: 224.0,
            ..page::Backdrop::default()
        }
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
        let page_index = self.page_index();
        match id {
            "demo" | "basic" => {
                let description = if id == "basic" {
                    "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                } else {
                    "This action cannot be undone. This will permanently delete your account from our servers."
                };
                alert_dialog(show, move |a| {
                    column![
                        a.header("Are you absolutely sure?", description,),
                        a.footer([
                            a.cancel(button("Cancel").style(button::Outline)),
                            a.action(button("Continue")),
                        ]),
                    ]
                    .width(Fill)
                    .into()
                })
                .into()
            }

            "small" => alert_dialog(show, |a| {
                column![
                    a.header(
                        "Allow accessory to connect?",
                        "Do you want to allow the USB accessory to connect to this device?",
                    ),
                    a.footer([
                        a.cancel(button("Don't allow").style(button::Outline)),
                        a.action(button("Allow")),
                    ]),
                ]
                .width(Fill)
                .into()
            })
            .size(Size::Sm)
            .into(),

            "media" => alert_dialog(
                |a| a.trigger(button("Share Project").style(button::Outline)),
                |a| {
                    column![
                        a.header(
                            "Share this project?",
                            "Anyone with the link will be able to view and edit this project.",
                        )
                        .media(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 2a10 10 0 0 1 7.38 16.75" /> <path d="M12 8v8" /> <path d="M16 12H8" /> <path d="M2.5 8.875a10 10 0 0 0-.5 3" /> <path d="M2.83 16a10 10 0 0 0 2.43 3.4" /> <path d="M4.636 5.235a10 10 0 0 1 .891-.857" /> <path d="M8.644 21.42a10 10 0 0 0 7.631-.38" /> </svg>"#.as_bytes()))
                .width(24)
                .height(24)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
                .boxed()),
                        a.footer([
                            a.cancel(button("Cancel").style(button::Outline)),
                            a.action(button("Share")),
                        ]),
                    ]
                    .width(Fill)
                    .into()
                },
            )
            .into(),

            "small-media" => alert_dialog(show, |a| {
                column![
                    a.header(
                        "Allow accessory to connect?",
                        "Do you want to allow the USB accessory to connect to this device?",
                    )
                    .media(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m7 7 10 10-5 5V2l5 5L7 17" /> </svg>"#.as_bytes()))
                .width(24)
                .height(24)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
                .boxed()),
                    a.footer([
                        a.cancel(button("Don't allow").style(button::Outline)),
                        a.action(button("Allow")),
                    ]),
                ]
                .width(Fill)
                .into()
            })
            .size(Size::Sm)
            .into(),

            "destructive" => alert_dialog(
                |a| a.trigger(button("Delete Chat").style(button::Destructive)),
                move |a| {
                    let description = rich_text([
                        span("This will permanently delete this chat conversation. View "),
                        span("Settings").link(()).underline(true),
                        span(" delete any memories saved during this chat."),
                    ])
                    .selectable(true)
                    .size(14)
                    .line_height(LineHeight::Absolute(20.0.into()))
                    .align_x(iced::Center)
                    .style(|theme| muted(theme).into())
                    .on_link_click(move |()| Message::Page(page_index, page::Message::ScrollTo(0)))
                    .boxed();
                    column![
                        a.header_content("Delete chat?", description,)
                            .media(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /> <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" /> <line x1="10" x2="10" y1="11" y2="17" /> <line x1="14" x2="14" y1="11" y2="17" /> </svg>"#.as_bytes()))
                .width(24)
                .height(24)
                .style(|theme: &Theme, _| svg::Style { color: Some(theme.destructive.base.into()) })
                .boxed())
                            .media_style(alert_dialog::Destructive),
                        a.footer([
                            a.cancel(button("Cancel").style(button::Outline)),
                            a.action(button("Delete").style(button::Destructive)),
                        ]),
                    ]
                    .width(Fill)
                    .into()
                },
            )
            .size(Size::Sm)
            .into(),

            _ => unreachable!("unknown alert dialog preview id: {id}"),
        }
    }
}
