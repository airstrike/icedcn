mod buttons;

use iced::{Fill, Task};

use icedxdy::widget::input_group::Align;
use icedxdy::widget::{Column, input, input_group, kbd, row, textarea};
use icedxdy::widget::{button, dropdown_menu, field, menu, space, spinner};
use icedxdy::{Slot, Theme};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Input Group",
    description: "Display additional information or actions to an input or textarea.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/input-group-demo.md",
    },
    page::Block::Md("examples/browser/snippets/input-group-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "inline-start",
                title: "inline-start",
                description: None,
                snippet: "examples/browser/snippets/input-group-inline-start.md",
            },
            page::Block::Example {
                id: "inline-end",
                title: "inline-end",
                description: None,
                snippet: "examples/browser/snippets/input-group-inline-end.md",
            },
            page::Block::Example {
                id: "block-start",
                title: "block-start",
                description: None,
                snippet: "examples/browser/snippets/input-group-block-start.md",
            },
            page::Block::Example {
                id: "block-end",
                title: "block-end",
                description: None,
                snippet: "examples/browser/snippets/input-group-block-end.md",
            },
            page::Block::Example {
                id: "icon",
                title: "Icon",
                description: None,
                snippet: "examples/browser/snippets/input-group-icon.md",
            },
            page::Block::Example {
                id: "text",
                title: "Text",
                description: Some("Display additional text information alongside inputs."),
                snippet: "examples/browser/snippets/input-group-text.md",
            },
            page::Block::Example {
                id: "button",
                title: "Button",
                description: Some("Add buttons to perform actions within the input group."),
                snippet: "examples/browser/snippets/input-group-button.md",
            },
            page::Block::Example {
                id: "kbd",
                title: "Kbd",
                description: None,
                snippet: "examples/browser/snippets/input-group-kbd.md",
            },
            page::Block::Example {
                id: "dropdown",
                title: "Dropdown",
                description: None,
                snippet: "examples/browser/snippets/input-group-dropdown.md",
            },
            page::Block::Example {
                id: "spinner",
                title: "Spinner",
                description: None,
                snippet: "examples/browser/snippets/input-group-spinner.md",
            },
            page::Block::Example {
                id: "textarea",
                title: "Textarea",
                description: None,
                snippet: "examples/browser/snippets/input-group-textarea.md",
            },
            page::Block::Example {
                id: "custom",
                title: "Custom Input",
                description: None,
                snippet: "examples/browser/snippets/input-group-custom.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/input-group-api-reference.md"),
];

pub struct InputGroupPage {
    state: page::State,
}

impl InputGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// The examples' `grid w-full max-w-sm gap-6` column.
fn stack<'a>(
    groups: impl IntoIterator<Item = Slot<'a, Message>>,
) -> iced::Element<'a, Message, Theme> {
    Column::with_children(groups)
        .spacing(24)
        .width(Fill)
        .max_width(384.0)
        .into()
}

/// An addon after the control.
fn end<'a>(content: impl Into<Slot<'a, Message>>) -> input_group::Addon<'a, Message> {
    input_group::addon(content).align(input_group::Align::InlineEnd)
}

impl page::Page for InputGroupPage {
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
            "demo" => Column::with_children([input_group(input("Search..."))
                .addon(input_group::addon(icon::search()))
                .addon(end("12 results"))
                .into()])
            .width(Fill)
            .max_width(320.)
            .into(),

            "icon" => stack([
                input_group(input("Search..."))
                    .addon(input_group::addon(icon::search()))
                    .into(),
                input_group(input("Enter your email"))
                    .addon(input_group::addon(icon::mail()))
                    .into(),
                input_group(input("Card number"))
                    .addon(input_group::addon(icon::credit_card()))
                    .addon(end(icon::check()))
                    .into(),
                input_group(input("Card number"))
                    .addon(end(row![icon::star(), icon::info()].spacing(8)))
                    .into(),
            ]),

            "text" => stack([
                input_group(input("0.00"))
                    .addon(input_group::addon(input_group::text("$")))
                    .addon(end(input_group::text("USD")))
                    .into(),
                input_group(input("example.com"))
                    .addon(input_group::addon(input_group::text("https://")))
                    .addon(end(input_group::text(".com")))
                    .into(),
                input_group(input("Enter your username"))
                    .addon(end(input_group::text("@company.com")))
                    .into(),
                input_group(textarea("Enter your message"))
                    .addon(
                        input_group::addon(input_group::text("120 characters left").size(12))
                            .align(input_group::Align::BlockEnd),
                    )
                    .into(),
            ]),

            "button" => buttons::preview(),

            "kbd" => stack([input_group(input("Search..."))
                .addon(input_group::addon(icon::search()))
                .addon(end(kbd("⌘K")))
                .into()]),

            "inline-start" => stack([field(
                input_group(input("Search...")).addon(input_group::addon(icon::search())),
            )
            .label("Input")
            .description("Icon positioned at the start.")
            .into()]),
            "inline-end" => stack([field(
                input_group(input("Enter password").secure(true)).addon(end(icon::eye_off())),
            )
            .label("Input")
            .description("Icon positioned at the end.")
            .into()]),
            "block-start" => stack([
                field(input_group(input("Enter your name")).addon(
                    input_group::addon(input_group::text("Full Name")).align(Align::BlockStart),
                ))
                .label("Input")
                .description("Header positioned above the input.")
                .into(),
                field(
                    input_group(textarea("console.log('Hello, world!');")).addon(
                        input_group::addon(
                            row![
                                icon::file_code(),
                                input_group::text("main.rs"),
                                space::horizontal(),
                                input_group::button(icon::copy()).square()
                            ]
                            .spacing(8),
                        )
                        .align(Align::BlockStart),
                    ),
                )
                .label("Textarea")
                .description("Header positioned above the textarea.")
                .into(),
            ]),
            "block-end" => stack([
                field(
                    input_group(input("Enter amount"))
                        .addon(input_group::addon(input_group::text("USD")).align(Align::BlockEnd)),
                )
                .label("Input")
                .description("Footer positioned below the input.")
                .into(),
                field(
                    input_group(textarea("Write a comment...")).addon(
                        input_group::addon(row![
                            input_group::text("0/280"),
                            space::horizontal(),
                            input_group::button("Post").style(button::Default)
                        ])
                        .align(Align::BlockEnd),
                    ),
                )
                .label("Textarea")
                .description("Footer positioned below the textarea.")
                .into(),
            ]),
            "dropdown" => stack([
                input_group(input("Enter file name"))
                    .addon(end(dropdown_menu(
                        |d| d.trigger(input_group::button(icon::ellipsis()).square()),
                        |_| {
                            menu::group(
                                ["Settings", "Copy path", "Open location"]
                                    .map(|item| menu::item(item).into()),
                            )
                        },
                    )
                    .align(dropdown_menu::Align::End)))
                    .into(),
                input_group(input("Enter search query"))
                    .addon(end(dropdown_menu(
                        |d| {
                            d.trigger(input_group::button(
                                row!["Search In...", icon::chevron_down()].spacing(4),
                            ))
                        },
                        |_| {
                            menu::group(
                                ["Documentation", "Blog Posts", "Changelog"]
                                    .map(|item| menu::item(item).into()),
                            )
                        },
                    )
                    .align(dropdown_menu::Align::End)))
                    .into(),
            ]),
            "spinner" => stack([
                input_group(input("Searching..."))
                    .addon(end(spinner()))
                    .into(),
                input_group(input("Processing..."))
                    .addon(input_group::addon(spinner()))
                    .into(),
                input_group(input("Saving changes..."))
                    .addon(end(
                        row![input_group::text("Saving..."), spinner()].spacing(8)
                    ))
                    .into(),
                input_group(input("Refreshing data..."))
                    .addon(input_group::addon(spinner()))
                    .addon(end(input_group::text("Please wait...")))
                    .into(),
            ]),
            "textarea" => {
                stack([
                    input_group(textarea("console.log('Hello, world!');").min_height(200.))
                        .addon(
                            input_group::addon(
                                row![
                                    icon::file_code(),
                                    input_group::text("main.rs"),
                                    space::horizontal(),
                                    input_group::button(icon::refresh_ccw()).square(),
                                    input_group::button(icon::copy()).square()
                                ]
                                .spacing(4),
                            )
                            .align(Align::BlockStart),
                        )
                        .addon(
                            input_group::addon(row![
                                input_group::text("Line 1, Column 1"),
                                space::horizontal(),
                                input_group::button(
                                    row!["Run", icon::corner_down_left()].spacing(4)
                                )
                                .style(button::Default)
                            ])
                            .align(Align::BlockEnd),
                        )
                        .into(),
                ])
            }
            "custom" => stack([input_group(textarea("Autoresize textarea..."))
                .addon(
                    input_group::addon(row![
                        space::horizontal(),
                        input_group::button("Submit").style(button::Default)
                    ])
                    .align(Align::BlockEnd),
                )
                .into()]),
            _ => unreachable!("unknown input group preview id: {id}"),
        }
    }
}
