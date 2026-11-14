```rust
use iced::alignment::Horizontal::Right;
use icedcn::widget::table::Track::{Auto, Fixed, Flex};
use icedcn::widget::{column, table, text};

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
```
