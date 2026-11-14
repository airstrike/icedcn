use iced::Task;

use icedcn::Horizontal;
use icedcn::Theme;
use icedcn::widget::{field, pagination, row, select};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Pagination",
    description: "Pagination with page navigation, next and previous links.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/pagination-demo.md",
    },
    page::Block::Md("examples/browser/snippets/pagination-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "simple",
                title: "Simple",
                description: Some("A simple pagination with only page numbers."),
                snippet: "examples/browser/snippets/pagination-simple.md",
            },
            page::Block::Example {
                id: "icons-only",
                title: "Icons Only",
                description: Some("Use icon buttons beside a rows-per-page selector."),
                snippet: "examples/browser/snippets/pagination-icons-only.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/pagination-api-reference.md"),
];

pub struct PaginationPage {
    state: page::State,
}

impl PaginationPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for PaginationPage {
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
            "demo" => pagination([
                pagination::previous().icon(icon::chevron_left()).into(),
                pagination::link("1").into(),
                pagination::link("2").active(true).into(),
                pagination::link("3").into(),
                pagination::ellipsis().icon(icon::ellipsis()).into(),
                pagination::next().icon(icon::chevron_right()).into(),
            ])
            .into(),

            "simple" => pagination([
                pagination::link("1").into(),
                pagination::link("2").active(true).into(),
                pagination::link("3").into(),
                pagination::link("4").into(),
                pagination::link("5").into(),
            ])
            .into(),

            "icons-only" => row![
                field(
                    select(["10", "25", "50", "100"].map(|count| select::item(count, count)))
                        .default_value("25")
                        .icon(icon::chevron_down)
                        .width(80)
                )
                .label("Rows per page")
                .orientation(Horizontal)
                .width(iced::Shrink),
                pagination([
                    pagination::link(icon::chevron_left())
                        .on_press(Message::Page(
                            self.page_index(),
                            page::Message::LinkClicked("#".into())
                        ))
                        .into(),
                    pagination::link(icon::chevron_right())
                        .on_press(Message::Page(
                            self.page_index(),
                            page::Message::LinkClicked("#".into())
                        ))
                        .into(),
                ])
                .width(iced::Shrink)
            ]
            .align_y(iced::Center)
            .spacing(16)
            .into(),

            _ => unreachable!("unknown pagination preview id: {id}"),
        }
    }
}
