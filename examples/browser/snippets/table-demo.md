```rust
use iced::alignment::Horizontal::Right;
use icedcn::widget::table::Track::{Auto, Fixed, Flex};
use icedcn::widget::{column, table, text};

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
```
