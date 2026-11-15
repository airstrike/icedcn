## Usage

Share the same column tracks across the header and body. Text cells inherit the table’s 14px type and 20px line height.

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
    let body = table::body([table::row([
        table::cell(text("INV001").medium()),
        table::cell("Paid"),
        table::cell("Credit Card"),
        table::cell("$250.00").align_x(Right),
    ])]);
    let grid = table::grid([Auto; 4]).header(header).body(body);
    table([
        Slot::from(grid),
        table::caption("A list of your recent invoices.").into(),
    ])
    .into()
}
```
