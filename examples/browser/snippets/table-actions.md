```rust
use iced::alignment::Horizontal::Right;
use iced::{Fill, FillPortion};
use icedxdy::Slot;
use icedxdy::widget::{button, column, dropdown_menu, menu, table, text};

let products = [
    ("Wireless Mouse", "$29.99"),
    ("Mechanical Keyboard", "$129.99"),
    ("USB-C Hub", "$49.99"),
];

let header = table::header([table::row([
    table::head("Product").width(FillPortion(2)),
    table::head("Price").width(Fill),
    table::head("Actions").width(Fill).align_x(Right),
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
        table::cell(text::small(name).medium()).width(FillPortion(2)),
        table::cell(text::small(price)).width(Fill),
        table::cell(actions).width(Fill).align_x(Right),
    ])
}));

let parts: [Slot<'_, Message>; 2] = [header.into(), body.into()];
table::table(parts).width(Fill)
```
