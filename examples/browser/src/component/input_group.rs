mod buttons;

use iced::{Fill, Task};

use icedcn::core::radius;
use icedcn::theme::text_input;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{button, dropdown_menu, field, menu, space, spinner};
use icedcn::widget::{column, container, input, input_group, kbd, row, textarea};
use icedcn::{Size, Slot, Theme};

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
    page::Block::Md("examples/browser/snippets/input-group-installation.md"),
    page::Block::Md("examples/browser/snippets/input-group-usage.md"),
    page::Block::Md("examples/browser/snippets/input-group-composition.md"),
    page::Block::Group {
        title: "Align",
        children: &[
            page::Block::Example {
                id: "inline-start",
                title: "inline-start",
                description: Some("The default addon sits before the control."),
                snippet: "examples/browser/snippets/input-group-inline-start.md",
            },
            page::Block::Example {
                id: "inline-end",
                title: "inline-end",
                description: Some("Place the addon after the control with `Align::InlineEnd`."),
                snippet: "examples/browser/snippets/input-group-inline-end.md",
            },
            page::Block::Example {
                id: "block-start",
                title: "block-start",
                description: Some("Place the addon above the control with `Align::BlockStart`."),
                snippet: "examples/browser/snippets/input-group-block-start.md",
            },
            page::Block::Example {
                id: "block-end",
                title: "block-end",
                description: Some("Place the addon below the control with `Align::BlockEnd`."),
                snippet: "examples/browser/snippets/input-group-block-end.md",
            },
        ],
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
        description: Some(
            "The textarea grows with its content. Its footer remains below the editor.",
        ),
        snippet: "examples/browser/snippets/input-group-custom.md",
    },
    page::Block::Md("examples/browser/snippets/input-group-api-reference.md"),
];

pub struct InputGroupPage {
    state: page::State,
    text_size: f32,
}

impl InputGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            text_size: 14.,
        })
    }
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

    fn window_width(&mut self, width: f32) {
        self.text_size = if width < 768. { 16. } else { 14. };
    }

    fn example_backdrop(&self, id: &str) -> page::Backdrop {
        let min_height = match id {
            "demo" | "block-end" => 416.,
            "inline-start" | "inline-end" => 192.,
            "block-start" | "textarea" => 384.,
            "icon" | "text" | "spinner" => 320.,
            "kbd" => 160.,
            "dropdown" | "custom" => 224.,
            _ => 288.,
        };
        page::Backdrop {
            min_height,
            padding: [0, 40].into(),
            ..Default::default()
        }
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let size = self.text_size;
        match id {
            "demo" => demo(size).into(),
            "icon" => icons(size).into(),
            "text" => text_addons(size).into(),
            "button" => buttons::preview(size),
            "kbd" => shortcut(size).into(),
            "inline-start" => inline_start(size).into(),
            "inline-end" => inline_end(size).into(),
            "block-start" => block_start(size).into(),
            "block-end" => block_end(size).into(),
            "dropdown" => dropdowns(size).into(),
            "spinner" => spinners(size).into(),
            "textarea" => editor(size).into(),
            "custom" => custom(size).into(),
            _ => unreachable!("unknown input group preview id: {id}"),
        }
    }
}

fn demo<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    container(
        input_group(input("Search...").text_size(size))
            .addon(addon(icon::search().size(16).line_height(1.0)))
            .addon(addon("12 results").align(Align::InlineEnd)),
    )
    .width(Fill)
    .max_width(320.)
    .into()
}

fn icons<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        input_group(input("Search...").text_size(size))
            .addon(addon(icon::search().size(16).line_height(1.0))),
        input_group(input("Enter your email").text_size(size))
            .addon(addon(icon::mail().size(16).line_height(1.0))),
        input_group(input("Card number").text_size(size))
            .addon(addon(icon::credit_card().size(16).line_height(1.0),))
            .addon(addon(icon::check().size(16).line_height(1.0)).align(Align::InlineEnd),),
        input_group(input("Card number").text_size(size)).addon(
            addon(
                row![
                    icon::star().size(16).line_height(1.0),
                    icon::info().size(16).line_height(1.0)
                ]
                .spacing(8),
            )
            .align(Align::InlineEnd),
        ),
    ]
    .spacing(24)
    .width(Fill)
    .max_width(384.)
    .into()
}

fn text_addons<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        input_group(input("0.00").text_size(size))
            .addon(addon(input_group::text("$")))
            .addon(addon(input_group::text("USD")).align(Align::InlineEnd)),
        input_group(input("example.com").text_size(size))
            .addon(addon(input_group::text("https://")))
            .addon(addon(input_group::text(".com")).align(Align::InlineEnd)),
        input_group(input("Enter your username").text_size(size))
            .addon(addon(input_group::text("@company.com")).align(Align::InlineEnd)),
        input_group(textarea("Enter your message").text_size(size)).addon(
            addon(
                input_group::text("120 characters left")
                    .size(12)
                    .line_height(iced::Pixels(16.)),
            )
            .align(Align::BlockEnd),
        ),
    ]
    .spacing(24)
    .width(Fill)
    .max_width(384.)
    .into()
}

fn shortcut<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    container(
        input_group(input("Search...").text_size(size))
            .addon(addon(icon::search().size(16).line_height(1.0)))
            .addon(addon(kbd("⌘K")).align(Align::InlineEnd)),
    )
    .width(Fill)
    .max_width(384.)
    .into()
}

fn inline_start<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    container(
        field(
            input_group(input("Search...").text_size(size))
                .addon(addon(icon::search().size(16).line_height(1.0))),
        )
        .label("Input")
        .description("Icon positioned at the start."),
    )
    .width(Fill)
    .max_width(384.)
    .into()
}

fn inline_end<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    container(
        field(
            input_group(input("Enter password").text_size(size).secure(true))
                .addon(addon(icon::eye_off().size(16).line_height(1.0)).align(Align::InlineEnd)),
        )
        .label("Input")
        .description("Icon positioned at the end."),
    )
    .width(Fill)
    .max_width(384.)
    .into()
}

fn block_start<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        field(
            input_group(input("Enter your name").text_size(size))
                .addon(addon(input_group::text("Full Name")).align(Align::BlockStart)),
        )
        .label("Input")
        .description("Header positioned above the input."),
        field(
            input_group(
                textarea("console.log('Hello, world!');")
                    .text_size(14.)
                    .font(iced::Font::MONOSPACE),
            )
            .addon(
                addon(
                    row![
                        icon::file_code().size(16).line_height(1.0),
                        input_group::text("script.js").font(iced::Font::MONOSPACE),
                        space::horizontal(),
                        input_group::button(icon::copy().size(16).line_height(1.0)).square()
                    ]
                    .spacing(8)
                    .width(Fill),
                )
                .align(Align::BlockStart),
            ),
        )
        .label("Textarea")
        .description("Header positioned above the textarea."),
    ]
    .spacing(20)
    .width(Fill)
    .max_width(384.)
    .into()
}

fn block_end<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        field(
            input_group(input("Enter amount").text_size(size))
                .addon(addon(input_group::text("USD")).align(Align::BlockEnd)),
        )
        .label("Input")
        .description("Footer positioned below the input."),
        field(
            input_group(textarea("Write a comment...").text_size(size)).addon(
                addon(
                    row![
                        input_group::text("0/280"),
                        space::horizontal(),
                        input_group::button("Post")
                            .size(Size::Sm)
                            .height(32)
                            .style(button::Default)
                    ]
                    .width(Fill),
                )
                .align(Align::BlockEnd),
            ),
        )
        .label("Textarea")
        .description("Footer positioned below the textarea."),
    ]
    .spacing(20)
    .width(Fill)
    .max_width(384.)
    .into()
}

fn dropdowns<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        input_group(input("Enter file name").text_size(size)).addon(
            addon(
                dropdown_menu(
                    |d| {
                        d.trigger(
                            input_group::button(icon::ellipsis().size(16).line_height(1.0))
                                .square(),
                        )
                    },
                    |_| {
                        menu::group(
                            ["Settings", "Copy path", "Open location"]
                                .map(|item| menu::item(item).into()),
                        )
                    },
                )
                .align(dropdown_menu::Align::End),
            )
            .align(Align::InlineEnd),
        ),
        input_group(input("Enter search query").text_size(size))
            .style(text_input::Default + radius(16))
            .addon(
                addon(
                    dropdown_menu(
                        |d| {
                            d.trigger(input_group::button(
                                row![
                                    "Search In...",
                                    icon::chevron_down().size(16).line_height(1.0)
                                ]
                                .spacing(4),
                            ))
                        },
                        |_| {
                            menu::group(
                                ["Documentation", "Blog Posts", "Changelog"]
                                    .map(|item| menu::item(item).into()),
                            )
                        },
                    )
                    .align(dropdown_menu::Align::End),
                )
                .align(Align::InlineEnd),
            ),
    ]
    .spacing(16)
    .width(Fill)
    .max_width(384.)
    .into()
}

fn spinners<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        input_group(input("Searching...").text_size(size))
            .addon(addon(spinner().circle()).align(Align::InlineEnd)),
        input_group(input("Processing...").text_size(size)).addon(addon(spinner().circle())),
        input_group(input("Saving changes...").text_size(size)).addon(
            addon(row![input_group::text("Saving..."), spinner().circle()].spacing(8))
                .align(Align::InlineEnd),
        ),
        input_group(input("Refreshing data...").text_size(size))
            .addon(addon(spinner().spokes()))
            .addon(addon(input_group::text("Please wait...")).align(Align::InlineEnd)),
    ]
    .spacing(16)
    .width(Fill)
    .max_width(384.)
    .into()
}

fn editor<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    container(
        input_group(
            textarea("console.log('Hello, world!');")
                .text_size(size)
                .min_height(200.),
        )
        .addon(
            addon(
                row![
                    icon::file_code().size(16).line_height(1.0),
                    input_group::text("script.js").font(iced::Font::MONOSPACE),
                    space::horizontal(),
                    input_group::button(icon::refresh_ccw().size(16).line_height(1.0)).square(),
                    input_group::button(icon::copy().size(16).line_height(1.0)).square()
                ]
                .spacing(4)
                .width(Fill),
            )
            .align(Align::BlockStart)
            .divider(),
        )
        .addon(
            addon(
                row![
                    input_group::text("Line 1, Column 1"),
                    space::horizontal(),
                    input_group::button(
                        row!["Run", icon::corner_down_left().size(16).line_height(1.0)].spacing(4)
                    )
                    .size(Size::Sm)
                    .height(32)
                    .style(button::Default)
                ]
                .width(Fill),
            )
            .align(Align::BlockEnd)
            .divider(),
        ),
    )
    .width(Fill)
    .max_width(448.)
    .into()
}

fn custom<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    container(
        input_group(textarea("Autoresize textarea...").text_size(size)).addon(
            addon(
                row![
                    space::horizontal(),
                    input_group::button("Submit")
                        .size(Size::Sm)
                        .height(32)
                        .style(button::Default)
                ]
                .width(Fill),
            )
            .align(Align::BlockEnd),
        ),
    )
    .width(Fill)
    .max_width(384.)
    .into()
}
