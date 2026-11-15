//! Sales list, receipt details, and the shared receipt editor.
use iced::alignment::Horizontal::Right;
use iced::{Center, Element, Fill};
use icedcn::widget::table::Track::Auto;
use icedcn::widget::{badge, button, column, field, input, row, select, space, table, text};
use icedcn::{Slot, Theme, card};

use super::Message;
use super::receipt::{Receipt, Tax, Totals, currency};

pub fn list<'a>(sales: &'a [Receipt], query: &'a str) -> Element<'a, Message, Theme> {
    let needle = query.trim().to_lowercase();
    let filtered: Vec<_> = sales
        .iter()
        .filter(|sale| sale.name.to_lowercase().contains(&needle))
        .collect();
    let mut rows = column![].spacing(12).width(Fill);
    for sale in &filtered {
        rows = rows.push(
            button(
                row![
                    column![
                        text::label(&sale.name),
                        text::muted(format!(
                            "Receipt #{:04} · {} items",
                            sale.id,
                            sale.items.len()
                        ))
                    ]
                    .spacing(6)
                    .width(Fill),
                    text::large(currency(sale.totals.total)),
                    text::muted("→"),
                ]
                .spacing(16)
                .align_y(Center),
            )
            .style(button::Outline)
            .padding(20)
            .width(Fill)
            .height(86)
            .on_press(Message::Show(sale.id))
            .into(),
        );
    }
    if filtered.is_empty() {
        rows = rows.push(
            text::muted(if sales.is_empty() {
                "No receipts yet. Create your first sale to get started."
            } else {
                "No receipts match your search."
            })
            .into(),
        );
    }
    column![
        row![
            column![text::h1("Receipts"), text::muted("Your sales, down to the last cent.")].spacing(8).width(Fill),
            button("New sale").on_press(Message::New),
        ].spacing(16).align_y(Center),
        field(input("Search by sale name…").value(query).on_input(Message::Search).width(Fill)).label("Find a receipt"),
        rows,
        text::muted("A remake of iced_receipts. Sample tax rates are part of the demo; receipts are kept for this session."),
    ].spacing(24).width(Fill).into()
}

pub fn show(sale: &Receipt, width: f32) -> Element<'_, Message, Theme> {
    let items: Slot<'_, Message> = if width >= 700. {
        let header = table::header([table::row([
            table::head("Item"),
            table::head("Qty").align_x(Right),
            table::head("Price").align_x(Right),
            table::head("Tax group"),
            table::head("Amount").align_x(Right),
        ])]);
        let body = table::body(sale.items.iter().map(|item| {
            table::row([
                table::cell(text::label(&item.name)),
                table::cell(item.quantity.as_str()).align_x(Right),
                table::cell(currency(item.unit_price().unwrap_or(0))).align_x(Right),
                table::cell(item.tax.label()),
                table::cell(currency(item.amount().unwrap_or(0))).align_x(Right),
            ])
        }));
        table([Slot::from(table::grid([Auto; 5]).header(header).body(body))]).into()
    } else {
        column(sale.items.iter().map(|item| {
            row![
                column![
                    text::label(&item.name),
                    text::muted(format!(
                        "{} × {} · {}",
                        item.quantity,
                        currency(item.unit_price().unwrap_or(0)),
                        item.tax.label()
                    ))
                ]
                .spacing(4)
                .width(Fill),
                text::label(currency(item.amount().unwrap_or(0))),
            ]
            .spacing(12)
            .into()
        }))
        .spacing(20)
        .into()
    };
    column![
        row![
            button("← Sales")
                .style(button::Ghost)
                .on_press(Message::Back),
            space::horizontal(),
            button("Edit receipt")
                .style(button::Outline)
                .on_press(Message::Edit)
        ]
        .spacing(8),
        card(column![
            card::header(sale.name.as_str())
                .description(format!("Receipt #{:04}", sale.id))
                .action(badge("Saved").style(badge::Secondary)),
            card::content(items),
            card::footer(totals(sale.totals)),
        ])
        .width(Fill),
    ]
    .spacing(24)
    .width(Fill)
    .into()
}

pub fn edit<'a>(
    sale: &'a Receipt,
    editing: bool,
    error: Option<&'static str>,
    width: f32,
) -> Element<'a, Message, Theme> {
    let narrow = width < 700.;
    let mut items = column![].spacing(16).width(Fill);
    for item in &sale.items {
        let id = item.id;
        let name = field(
            input("Item name")
                .value(&item.name)
                .on_input(move |value| Message::ItemName(id, value))
                .on_submit(Message::NextField)
                .width(Fill),
        )
        .label("Item");
        let quantity = field(
            input("1")
                .value(&item.quantity)
                .on_input(move |value| Message::Quantity(id, value))
                .on_submit(Message::NextField)
                .width(Fill),
        )
        .label("Quantity");
        let price = field(
            input("0.00")
                .value(&item.price)
                .on_input(move |value| Message::Price(id, value))
                .on_submit(Message::NextField)
                .width(Fill),
        )
        .label("Unit price ($)");
        let tax = field(
            select(Tax::ALL.map(|tax| select::item(tax.key(), tax.label())))
                .value(item.tax.key())
                .on_value_change(move |value| Message::Tax(id, value))
                .width(Fill),
        )
        .label("Tax group");
        let fields: Slot<'_, Message> = if narrow {
            column![name, row![quantity, price].spacing(12), tax]
                .spacing(12)
                .width(Fill)
                .into()
        } else {
            row![name, quantity, price, tax]
                .spacing(12)
                .width(Fill)
                .into()
        };
        items = items.push(
            card(column![
                card::content(fields),
                card::footer(
                    row![
                        text::muted(format!(
                            "Line total: {}",
                            currency(item.amount().unwrap_or(0))
                        )),
                        space::horizontal(),
                        button("Remove item")
                            .style(button::Ghost)
                            .on_press(Message::RemoveItem(id)),
                    ]
                    .align_y(Center)
                    .spacing(8)
                ),
            ])
            .width(Fill)
            .into(),
        );
    }
    let service = field(
        input("0")
            .value(&sale.service)
            .on_input(Message::Service)
            .on_submit(Message::NextField)
            .width(Fill),
    )
    .label("Service charge (%)");
    let gratuity = field(
        input("0.00")
            .value(&sale.gratuity)
            .on_input(Message::Gratuity)
            .on_submit(Message::Save)
            .width(Fill),
    )
    .label("Gratuity ($)");
    let mut content = column![
        text::h2(if editing { "Edit receipt" } else { "New sale" }),
        field(
            input("For example, Friday lunch")
                .value(&sale.name)
                .on_input(Message::Name)
                .on_submit(Message::NextField)
                .width(Fill)
        )
        .label("Sale name"),
        items,
        button("+ Add item")
            .style(button::Outline)
            .on_press(Message::AddItem),
        card(column![
            card::header("Summary")
                .description("Tax applies to each item. Service charge applies to the subtotal."),
            card::content(row![service, gratuity].spacing(16).width(Fill)),
            card::footer(totals(sale.totals)),
        ])
        .width(Fill),
    ]
    .spacing(24)
    .width(Fill);
    if let Some(error) = error {
        content = content.push(text::error(error).into());
    }
    content
        .push(
            row![
                button("Cancel")
                    .style(button::Outline)
                    .on_press(Message::Back),
                space::horizontal(),
                button("Save receipt").on_press(Message::Save),
            ]
            .spacing(8)
            .into(),
        )
        .into()
}

fn totals(totals: Totals) -> Slot<'static, Message> {
    let mut lines = column![].spacing(10).width(Fill);
    for (label, amount) in [
        ("Subtotal", totals.subtotal),
        ("Tax", totals.tax),
        ("Service charge", totals.service),
        ("Gratuity", totals.gratuity),
    ] {
        lines = lines.push(
            row![
                text::muted(label),
                space::horizontal(),
                text::small(currency(amount))
            ]
            .spacing(16)
            .into(),
        );
    }
    lines
        .push(
            row![
                text::large("Total"),
                space::horizontal(),
                text::large(currency(totals.total))
            ]
            .spacing(16)
            .into(),
        )
        .into()
}
