// RTL is omitted under the component policy.
mod payments;

use crate::{Message, page};
use iced::Task;
use icedcn::Theme;
use payments::Payment;

pub static META: page::Meta = page::Meta {
    title: "Data Table",
    description: "An interactive data table with sorting, filtering, column visibility and row selection.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/data-table-demo.md",
    },
    page::Block::Md("examples/browser/snippets/data-table-composition.md"),
    page::Block::Md("examples/browser/snippets/data-table-contents.md"),
    page::Block::Md("examples/browser/snippets/data-table-prerequisites.md"),
    page::Block::Md("examples/browser/snippets/data-table-project.md"),
    page::Block::Md("examples/browser/snippets/data-table-setup.md"),
    page::Block::Group {
        title: "Basic Table",
        children: &[
            page::Block::Md("examples/browser/snippets/data-table-guide-basic-table.md"),
            page::Block::Md("examples/browser/snippets/data-table-build.md"),
            page::Block::Md("examples/browser/snippets/data-table-render.md"),
        ],
    },
    page::Block::Group {
        title: "Cell Formatting",
        children: &[page::Block::Md(
            "examples/browser/snippets/data-table-guide-cell-formatting.md",
        )],
    },
    page::Block::Group {
        title: "Row Actions",
        children: &[page::Block::Md(
            "examples/browser/snippets/data-table-guide-row-actions.md",
        )],
    },
    page::Block::Group {
        title: "Pagination",
        children: &[
            page::Block::Md("examples/browser/snippets/data-table-guide-pagination.md"),
            page::Block::Md("examples/browser/snippets/data-table-paging-controls.md"),
        ],
    },
    page::Block::Group {
        title: "Sorting",
        children: &[
            page::Block::Md("examples/browser/snippets/data-table-guide-sorting.md"),
            page::Block::Md("examples/browser/snippets/data-table-sort-header.md"),
        ],
    },
    page::Block::Group {
        title: "Filtering",
        children: &[page::Block::Md(
            "examples/browser/snippets/data-table-guide-filtering.md",
        )],
    },
    page::Block::Group {
        title: "Visibility",
        children: &[page::Block::Md(
            "examples/browser/snippets/data-table-guide-visibility.md",
        )],
    },
    page::Block::Group {
        title: "Row Selection",
        children: &[
            page::Block::Md("examples/browser/snippets/data-table-guide-row-selection.md"),
            page::Block::Md("examples/browser/snippets/data-table-select-page.md"),
            page::Block::Md("examples/browser/snippets/data-table-selected-rows.md"),
        ],
    },
    page::Block::Group {
        title: "Reusable Components",
        children: &[
            page::Block::Md("examples/browser/snippets/data-table-column-header.md"),
            page::Block::Md("examples/browser/snippets/data-table-pagination.md"),
            page::Block::Md("examples/browser/snippets/data-table-column-toggle.md"),
        ],
    },
];
const PAYMENTS: [Payment<'static>; 5] = [
    Payment {
        id: "m5gr84i9",
        amount: 316.00,
        status: "Success",
        email: "ken99@example.com",
    },
    Payment {
        id: "3u1reuv4",
        amount: 242.00,
        status: "Success",
        email: "abe45@example.com",
    },
    Payment {
        id: "derv1ws0",
        amount: 837.00,
        status: "Processing",
        email: "monserrat44@example.com",
    },
    Payment {
        id: "5kma53ae",
        amount: 874.00,
        status: "Success",
        email: "silas22@example.com",
    },
    Payment {
        id: "bhqecj4p",
        amount: 721.00,
        status: "Failed",
        email: "carmella@example.com",
    },
];
pub struct DataTablePage {
    state: page::State,
    wide: bool,
}

impl DataTablePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
        })
    }
}

impl page::Page for DataTablePage {
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
        self.wide = width >= 768.;
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: [40., if self.wide { 32. } else { 16. }].into(),
            ..Default::default()
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
        match id {
            "demo" => payments::payments(&PAYMENTS, Message::CopyText).into(),
            _ => unreachable!("unknown data-table preview id: {id}"),
        }
    }
}
