use iced::alignment::Horizontal::Right;
use iced::{Fill, Task};
use icedcn::widget::table::Track::Auto;

use icedcn::widget::{button, container, dropdown_menu, menu, table, text};
use icedcn::{Slot, Theme};

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
    page::Block::Md("examples/browser/snippets/table-usage.md"),
    page::Block::Md("examples/browser/snippets/table-composition.md"),
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
    page::Block::Md("examples/browser/snippets/table-guide-data-table.md"),
    page::Block::Md("examples/browser/snippets/table-api-reference.md"),
];

/// Invoice number, payment status, method and amount.
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

/// Render the provided invoice records and the statement total.
fn invoices<'a, M: 'a>(records: &'a [[&'a str; 4]]) -> Slot<'a, M> {
    let header = table::header([table::row([
        table::head("Invoice").width(100),
        table::head("Status"),
        table::head("Method"),
        table::head("Amount").align_x(Right),
    ])]);
    let body = table::body(records.iter().map(|&[invoice, status, method, amount]| {
        table::row([
            table::cell(text(invoice).medium()),
            table::cell(status),
            table::cell(method),
            table::cell(amount).align_x(Right),
        ])
    }));
    let footer = table::footer([table::row([
        table::cell("Total").span(3),
        table::cell("$2,500.00").align_x(Right),
    ])]);
    let grid = table::grid([Auto; 4])
        .header(header)
        .body(body)
        .footer(footer);
    table([
        Slot::from(grid),
        table::caption("A list of your recent invoices.").into(),
    ])
    .into()
}

fn actions<'a, M: Clone + 'static>() -> Slot<'a, M> {
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
            |menu| {
                menu.trigger(
                    button(icon::ellipsis().size(16).line_height(1.0))
                        .square()
                        .style(button::Ghost)
                        .has_popup(true),
                )
            },
            |_| {
                menu::group![
                    menu::item("Edit"),
                    menu::item("Duplicate"),
                    menu::separator(),
                    menu::item("Delete").style(menu::Destructive),
                ]
            },
        )
        .align(dropdown_menu::Align::End);

        table::row([
            table::cell(text(name).medium()),
            table::cell(price),
            table::cell(actions).align_x(Right),
        ])
    }));

    table([table::grid([Auto; 3]).header(header).body(body)]).into()
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

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: [0., 40.].into(),
            ..Default::default()
        }
    }
    fn docs_preview(&self, id: &str, outer: &Theme) -> iced::Element<'_, Message, Theme> {
        let height = if id == "demo" { 480. } else { 288. };
        container(self.themed_preview(id, outer))
            .center_x(Fill)
            .center_y(height)
            .into()
    }
    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => invoices(&INVOICES).into(),

            "footer" => invoices(&INVOICES[..3]).into(),

            "actions" => actions().into(),

            _ => unreachable!("unknown table preview id: {id}"),
        }
    }
}
