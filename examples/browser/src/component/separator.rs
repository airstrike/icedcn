// Intentional skip per AGENTS.md: separator-rtl (RTL).

use iced::widget::responsive;
use iced::{Center, Fill, Length, Pixels, Shrink, Task, Widget};

use icedxdy::widget::{Column, column, container, row, separator, space, text};
use icedxdy::{Slot, Theme, Vertical, theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Separator",
    description: "Scroll through content inside a fixed viewport.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/separator-demo.md",
    },
    page::Block::Md("examples/browser/snippets/separator-installation.md"),
    page::Block::Md("examples/browser/snippets/separator-usage.md"),
    page::Block::Example {
        id: "vertical",
        title: "Vertical",
        description: Some("Use `.orientation(Vertical)` for a vertical separator."),
        snippet: "examples/browser/snippets/separator-vertical.md",
    },
    page::Block::Example {
        id: "menu",
        title: "Menu",
        description: Some("Vertical separators between menu items with descriptions."),
        snippet: "examples/browser/snippets/separator-menu.md",
    },
    page::Block::Example {
        id: "list",
        title: "List",
        description: Some("Horizontal separators between list items."),
        snippet: "examples/browser/snippets/separator-list.md",
    },
    page::Block::Md("examples/browser/snippets/separator-api-reference.md"),
];

pub struct SeparatorPage {
    state: page::State,
    wide: bool,
}

impl SeparatorPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
        })
    }
}

/// A menu entry: a medium title over an `text-xs` muted description.
fn menu_item<'a>(title: &'a str, description: &'a str) -> Column<'a, Message> {
    column![
        text::small(title).medium().line_height(Pixels(20.0)),
        text::xs(description)
            .line_height(Pixels(16.0))
            .wrapping(text::Wrapping::Word)
            .style(theme::text::Muted),
    ]
    .spacing(4)
}

/// A `<dl>` row: the term on the left, its muted value on the right.
fn list_item<'a>(term: &'a str, value: &'a str) -> Slot<'a, Message> {
    row![
        text::small(term).line_height(Pixels(20.0)),
        space::horizontal(),
        text::muted(value).line_height(Pixels(20.0)),
    ]
    .width(Fill)
    .into()
}

impl page::Page for SeparatorPage {
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

    fn window_width(&mut self, width: f32) {
        self.wide = width >= 768.0;
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => column![
                column![
                    text::label("icedxdy").height(14),
                    text::muted("The Foundation for your Design System").line_height(Pixels(20.0)),
                ]
                .spacing(6),
                separator(),
                text::small(
                    "A set of beautifully designed components that you can customize, \
                     extend, and build on.",
                )
                .line_height(Pixels(20.0)),
            ]
            .spacing(16)
            .width(Fill)
            .max_width(384.0)
            .into(),

            "vertical" => row![
                text::small("Blog").line_height(Pixels(20.0)),
                separator().orientation(Vertical),
                text::small("Docs").line_height(Pixels(20.0)),
                separator().orientation(Vertical),
                text::small("Source").line_height(Pixels(20.0)),
            ]
            .spacing(16)
            .height(20)
            .align_y(Center)
            .into(),

            // Only the window controls `md:` visibility; narrow embeds share column space.
            "menu" => {
                let wide = self.wide;
                responsive(move |bounds| {
                    let cramped = wide && bounds.width < 355.0;
                    let width = |portion| {
                        if cramped {
                            Length::FillPortion(portion)
                        } else {
                            Shrink
                        }
                    };
                    let mut items = row![
                        menu_item("Settings", "Manage preferences").width(width(6)),
                        separator().orientation(Vertical),
                        menu_item("Account", "Profile & security").width(width(5)),
                    ]
                    .spacing(if wide { 16 } else { 8 })
                    .width(if cramped { Fill } else { Shrink })
                    .align_y(Center);
                    if wide {
                        items = items
                            .push(separator().orientation(Vertical).into())
                            .push(menu_item("Help", "Support & docs").width(width(4)).into());
                    }
                    iced::Element::from(container(items).center_x(Fill))
                })
                .height(Shrink)
                .boxed()
            }

            "list" => column![
                list_item("Item 1", "Value 1"),
                separator(),
                list_item("Item 2", "Value 2"),
                separator(),
                list_item("Item 3", "Value 3"),
            ]
            .spacing(8)
            .width(Fill)
            .max_width(384.0)
            .into(),

            _ => unreachable!("unknown separator preview id: {id}"),
        }
    }
}
