```rust
use iced::{Element, Fill};
use icedxdy::widget::text;
use sweeten::widget::table;

let rows = &[
    ("Empty", "Overflowing"),
    ("Modest", "Satisfied"),
    ("Full", "Ecstatic"),
];

let header = |label| Element::from(text::p(label).bold());

let columns = [
    table::column(Some(header("King's Treasury")), |row: &(_, _)| Element::from(text::p(row.0)))
        .width(Fill),
    table::column(Some(header("People's happiness")), |row: &(_, _)| Element::from(text::p(row.1)))
        .width(Fill),
];

table::table(columns, rows)
    .width(Fill)
    .padding_x(16)
    .padding_y(8)
    .separator(1)
    .border(1)
```
