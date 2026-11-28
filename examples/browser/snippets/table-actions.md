```rust
use crate::icon;
use iced::alignment::Horizontal::Right;
use icedcn::View;
use icedcn::widget::table::Track::Auto;
use icedcn::widget::{button, dropdown_menu, menu, table, text};

pub fn actions<'a, M: Clone + 'static>() -> View<'a, M> {
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
```
