```rust
use iced::alignment::Horizontal::Right;
use icedcn::Slot;
use icedcn::widget::table::Track::Auto;
use icedcn::widget::{table, text};

pub fn view<'a, M: 'a>() -> Slot<'a, M> {
    let header = table::header([table::row([
        table::head("Invoice").width(100),
        table::head("Status"),
        table::head("Method"),
        table::head("Amount").align_x(Right),
    ])]);
    let body = table::body(INVOICES.into_iter().map(|[invoice, status, method, amount]| {
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

const INVOICES: [[&str; 4]; 7] = [
    ["INV001", "Paid", "Credit Card", "$250.00"],
    ["INV002", "Pending", "PayPal", "$150.00"],
    ["INV003", "Unpaid", "Bank Transfer", "$350.00"],
    ["INV004", "Paid", "Credit Card", "$450.00"],
    ["INV005", "Paid", "PayPal", "$550.00"],
    ["INV006", "Pending", "Bank Transfer", "$200.00"],
    ["INV007", "Unpaid", "Credit Card", "$300.00"],
];
```
