//! Edit new and existing sales.
use super::super::operation;
use crate::icon;
use iced::alignment::Horizontal::{Center, Right};
use iced::widget::Id;
use iced::{Alignment, Element, Fill};
use icedcn::widget::scrollable::scrollable;
use icedcn::widget::table::Track::Auto;
use icedcn::widget::{button, column, container, input, row, select, space, table, text, tooltip};
use icedcn::{Slot, Theme, card, separator};

use super::super::Hotkey;
use super::{Action, Instruction, Sale, TaxGroup};

#[derive(Debug, Clone)]
pub enum Message {
    NameInput(String),
    NameSubmit,
    AddItem,
    RemoveItem(usize),
    UpdateItem(usize, Field),
    SubmitItem(usize),
    UpdateServiceCharge(f32),
    UpdateGratuity(f32),
    Save,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum Field {
    Name(String),
    Price(String),
    Quantity(String),
    TaxGroup(TaxGroup),
}

pub fn view(sale: &Sale) -> Element<'_, Message, Theme> {
    let header = row![
        space::horizontal().width(40),
        input("Sale Name")
            .id("receipts-sale-name")
            .value(&sale.name)
            .on_input(Message::NameInput)
            .on_submit(Message::NameSubmit)
            .width(Fill),
        space::horizontal(),
        row![
            button("Cancel")
                .on_press(Message::Cancel)
                .style(button::Outline),
            button("Save").on_press(Message::Save),
        ]
        .spacing(10),
    ]
    .spacing(5)
    .align_y(Alignment::Center)
    .width(Fill);

    let headings = table::header([table::row([
        table::head("Item Name"),
        table::head("Qty").align_x(Center).width(96),
        table::head("Price").align_x(Right).width(116),
        table::head("Tax Group").width(156),
        table::head("Total").align_x(Right).width(116),
        table::head("").width(51),
    ])]);
    let body = table::body(sale.items.iter().map(|item| {
        table::row([
            table::cell(
                container(
                    input("Item name")
                        .value(&item.name)
                        .id(form_id("name", item.id))
                        .on_input(|value| Message::UpdateItem(item.id, Field::Name(value)))
                        .on_submit(Message::SubmitItem(item.id))
                        .width(Fill),
                )
                .width(Fill)
                .min_width(160),
            ),
            table::cell(
                input("Quantity")
                    .value(item.quantity_string())
                    .id(form_id("quantity", item.id))
                    .align_x(Alignment::Center)
                    .on_input(|value| Message::UpdateItem(item.id, Field::Quantity(value)))
                    .on_submit(Message::SubmitItem(item.id))
                    .width(80),
            )
            .align_x(Center),
            table::cell(
                input("Price")
                    .value(item.price_string())
                    .id(form_id("price", item.id))
                    .align_x(Alignment::End)
                    .on_input(|value| Message::UpdateItem(item.id, Field::Price(value)))
                    .on_submit(Message::SubmitItem(item.id))
                    .width(100),
            )
            .align_x(Right),
            table::cell(
                select(
                    TaxGroup::ALL.map(|group| select::item(group.to_string(), group.to_string())),
                )
                .value(item.tax_group.to_string())
                .on_value_change(move |value| {
                    let group = TaxGroup::ALL
                        .into_iter()
                        .find(|group| group.to_string() == value)
                        .expect("select publishes one of its tax groups");
                    Message::UpdateItem(item.id, Field::TaxGroup(group))
                })
                .width(140),
            ),
            table::cell(text::small(format!("${:.2}", item.price() * item.quantity())).medium())
                .align_x(Right),
            table::cell(tooltip(
                button(icon::trash_2().size(16))
                    .width(35)
                    .style(button::Ghost)
                    .on_press(Message::RemoveItem(item.id)),
                "Remove item",
            ))
            .align_x(Right),
        ])
    }));
    let items_list = table([Slot::from(
        table::grid([Auto; 6]).header(headings).body(body),
    )]);

    let totals = column![
        row![
            text::muted("Subtotal").width(150),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_subtotal()))
        ]
        .height(32)
        .align_y(Alignment::Center)
        .width(Fill),
        row![
            text::muted("Service Charge").width(150),
            row![
                input("0.0")
                    .value(
                        sale.service_charge_percent
                            .map_or(String::new(), |p| format!("{:.1}", p))
                    )
                    .width(60)
                    .on_input(|value| Message::UpdateServiceCharge(if value.is_empty() {
                        0.0
                    } else {
                        value.parse().ok().unwrap_or(0.0)
                    }))
                    .on_submit(Message::Save),
                text("%"),
            ]
            .spacing(5),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_service_charge())),
        ]
        .height(32)
        .align_y(Alignment::Center)
        .width(Fill),
        row![
            text::muted("Tax").width(150),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_tax()))
        ]
        .height(32)
        .align_y(Alignment::Center)
        .width(Fill),
        row![
            text::muted("Gratuity").width(150),
            input("0.00")
                .value(
                    sale.gratuity_amount
                        .map_or(String::new(), |g| format!("{:.2}", g))
                )
                .width(100)
                .on_input(|value| Message::UpdateGratuity(if value.is_empty() {
                    0.0
                } else {
                    value.parse().ok().unwrap_or(0.0)
                }))
                .on_submit(Message::Save),
            space::horizontal(),
            text(format!("${:.2}", sale.gratuity_amount.unwrap_or(0.0))),
        ]
        .height(32)
        .align_y(Alignment::Center)
        .width(Fill),
        separator(),
        row![
            text::label("Total").width(150),
            space::horizontal(),
            text::large(format!("${:.2}", sale.calculate_total())).semibold()
        ]
        .height(32)
        .align_y(Alignment::Center)
        .width(Fill),
    ]
    .spacing(4)
    .width(Fill);

    container(
        column![
            header,
            card(
                card::content(
                    scrollable(
                        column![
                            button("+ Add Item")
                                .style(button::Outline)
                                .on_press(Message::AddItem),
                            items_list,
                        ]
                        .spacing(12)
                        .width(Fill)
                    )
                    .width(Fill)
                    .height(Fill)
                )
                .height(Fill)
            )
            .height(Fill)
            .width(Fill),
            card(card::content(totals)).width(Fill),
        ]
        .spacing(20)
        .height(Fill)
        .width(Fill),
    )
    .padding(20)
    .id(super::super::ROOT)
    .height(Fill)
    .width(Fill)
    .into()
}

pub fn handle_hotkey(hotkey: Hotkey) -> Action<Instruction, Message> {
    match hotkey {
        Hotkey::Tab(modifier) if modifier.shift() => Action::task(operation::focus_previous()),
        Hotkey::Tab(_) => Action::task(operation::focus_next()),
        _ => Action::none(),
    }
}

pub fn form_id(field: &str, id: usize) -> Id {
    Id::from(format!("{field}-{id}"))
}
