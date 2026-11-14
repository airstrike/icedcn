use iced::alignment::Horizontal::Right;
use iced::{Fill, Task};
use icedcn::widget::table::Track::{Auto, Fixed, Flex};

use icedcn::Theme;
use icedcn::widget::{button, column, dropdown_menu, menu, table, text};

use crate::{Message, icon, page};

pub static META: page::Meta = page::Meta {
    title: "Table",
    description: "A responsive table component.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/table-demo.md",
    },
    page::Block::Md("examples/browser/snippets/table-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "footer",
                title: "Footer",
                description: Some("Use `table::footer(..)` to add a footer to the table."),
                snippet: "examples/browser/snippets/table-footer.md",
            },
            page::Block::Example {
                id: "actions",
                title: "Actions",
                description: Some(
                    "A table showing actions for each row using a `dropdown_menu` component.",
                ),
                snippet: "examples/browser/snippets/table-actions.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/table-guide-data-table.md"),
    page::Block::Md("examples/browser/snippets/table-api-reference.md"),
];

/// invoices: number, status, method and amount.
const INVOICES: [[&str; 4]; 7] = [
    ["INV001", "Paid", "Credit Card", "$250.00"],
    ["INV002", "Pending", "PayPal", "$150.00"],
    ["INV003", "Unpaid", "Bank Transfer", "$350.00"],
    ["INV004", "Paid", "Credit Card", "$450.00"],
    ["INV005", "Paid", "PayPal", "$550.00"],
    ["INV006", "Pending", "Bank Transfer", "$200.00"],
    ["INV007", "Unpaid", "Credit Card", "$300.00"],
];

pub struct TablePage {
    state: page::State,
}

impl TablePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// The invoice table over the first `count` invoices, with its footer.
fn invoices<'a>(count: usize) -> iced::Element<'a, Message, Theme> {
    let header = table::header([table::row([
        table::head("Invoice"),
        table::head("Status"),
        table::head("Method"),
        table::head("Amount").align_x(Right),
    ])]);
    let body = table::body(
        INVOICES
            .iter()
            .take(count)
            .map(|[invoice, status, method, amount]| {
                table::row([
                    table::cell(text::small(*invoice).medium()),
                    table::cell(text::small(*status)),
                    table::cell(text::small(*method)),
                    table::cell(text::small(*amount)).align_x(Right),
                ])
            }),
    );
    let footer = table::footer([table::row([
        table::cell(text::small("Total").medium()).span(3),
        table::cell(text::small("$2,500.00").medium()).align_x(Right),
    ])]);
    column![
        table::grid([Fixed(100.0), Flex(1), Flex(1), Auto])
            .header(header)
            .body(body)
            .footer(footer),
        table::caption("A list of your recent invoices."),
    ]
    .width(Fill)
    .into()
}

fn actions<'a>() -> iced::Element<'a, Message, Theme> {
    let products = [
        ("Wireless Mouse", "$29.99"),
        ("Mechanical Keyboard", "$129.99"),
        ("USB-C Hub", "$49.99"),
    ];

    let header = table::header([table::row([
        table::head("Product"),
        table::head("Price"),
        table::head("Actions").align_x(Right),
    ])]);

    let body = table::body(products.into_iter().map(|(name, price)| {
        let actions = dropdown_menu(
            |menu| menu.trigger(button(icon::ellipsis()).square().style(button::Ghost)),
            |_| {
                column![
                    menu::item("Edit"),
                    menu::item("Duplicate"),
                    menu::separator(),
                    menu::item("Delete").style(menu::Destructive),
                ]
                .width(Fill)
                .into()
            },
        )
        .align(dropdown_menu::Align::End);

        table::row([
            table::cell(text::small(name).medium()),
            table::cell(text::small(price)),
            table::cell(actions).align_x(Right),
        ])
    }));

    table::grid([Flex(2), Flex(1), Auto])
        .header(header)
        .body(body)
        .into()
}

impl page::Page for TablePage {
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
            "demo" => invoices(INVOICES.len()),

            "footer" => invoices(3),

            "actions" => actions(),

            _ => unreachable!("unknown table preview id: {id}"),
        }
    }
}
