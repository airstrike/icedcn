//! Edit new and existing sales.
use super::super::operation;
use iced::widget::Id;
use iced::{Alignment, Element, Fill};
use icedcn::widget::scrollable::scrollable;
use icedcn::widget::{button, column, container, input, row, select, space, text};
use icedcn::{Theme, theme};

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
            .padding(5)
            .width(Fill),
        space::horizontal(),
        row![
            button("Cancel")
                .on_press(Message::Cancel)
                .style(button::Destructive),
            button("Save").on_press(Message::Save),
        ]
        .spacing(10),
    ]
    .spacing(5)
    .align_y(Alignment::Center)
    .width(Fill);

    let column_headers = row![
        text("Item Name").width(Fill),
        text("Qty").align_x(Alignment::Center).width(80),
        text("Price").align_x(Alignment::End).width(100),
        text("Tax Group").width(140),
        text("Total").align_x(Alignment::End).width(100),
        space::horizontal().width(35),
    ]
    .spacing(5)
    .width(Fill);

    let items_list = sale.items.iter().fold(
        column![column_headers].spacing(5).width(Fill),
        |col, item| {
            col.push(
                container(
                    row![
                        input("Item name")
                            .value(&item.name)
                            .id(form_id("name", item.id))
                            .on_input(|value| Message::UpdateItem(item.id, Field::Name(value)))
                            .on_submit(Message::SubmitItem(item.id))
                            .width(Fill)
                            .padding(5),
                        input("Quantity")
                            .value(item.quantity_string())
                            .id(form_id("quantity", item.id))
                            .align_x(Alignment::Center)
                            .on_input(|value| Message::UpdateItem(item.id, Field::Quantity(value)))
                            .on_submit(Message::SubmitItem(item.id))
                            .width(80)
                            .padding(5),
                        input("Price")
                            .value(item.price_string())
                            .id(form_id("price", item.id))
                            .align_x(Alignment::End)
                            .on_input(|value| Message::UpdateItem(item.id, Field::Price(value)))
                            .on_submit(Message::SubmitItem(item.id))
                            .width(100)
                            .padding(5),
                        select(
                            TaxGroup::ALL
                                .map(|group| select::item(group.to_string(), group.to_string()))
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
                        text(format!("${:.2}", item.price() * item.quantity()))
                            .align_x(Alignment::End)
                            .width(100),
                        button(text("×").align_x(Alignment::Center))
                            .width(35)
                            .on_press(Message::RemoveItem(item.id))
                            .style(button::Destructive),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center)
                    .width(Fill),
                )
                .class(theme::container::CardMuted)
                .padding(0)
                .width(Fill)
                .into(),
            )
        },
    );

    let totals = column![
        row![
            text("Subtotal").width(150),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_subtotal()))
        ]
        .height(30)
        .width(Fill),
        row![
            text("Service Charge").width(150),
            row![
                input("0.0")
                    .value(
                        sale.service_charge_percent
                            .map_or(String::new(), |p| format!("{:.1}", p))
                    )
                    .width(60)
                    .padding(5)
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
        .height(30)
        .width(Fill),
        row![
            text("Tax").width(150),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_tax()))
        ]
        .height(30)
        .width(Fill),
        row![
            text("Gratuity").width(150),
            input("0.00")
                .value(
                    sale.gratuity_amount
                        .map_or(String::new(), |g| format!("{:.2}", g))
                )
                .width(100)
                .padding(5)
                .on_input(|value| Message::UpdateGratuity(if value.is_empty() {
                    0.0
                } else {
                    value.parse().ok().unwrap_or(0.0)
                }))
                .on_submit(Message::Save),
            space::horizontal(),
            text(format!("${:.2}", sale.gratuity_amount.unwrap_or(0.0))),
        ]
        .height(30)
        .width(Fill),
        row![
            text("Total").width(150).size(16),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_total())).size(16)
        ]
        .height(30)
        .width(Fill),
    ]
    .spacing(2)
    .width(Fill);

    container(
        column![
            header,
            container(
                scrollable(
                    column![button("+ Add Item").on_press(Message::AddItem), items_list,]
                        .spacing(10)
                        .padding(20)
                        .width(Fill)
                )
                .height(Fill)
                .width(Fill)
            )
            .height(Fill)
            .width(Fill)
            .class(theme::container::CardMuted),
            container(totals)
                .padding(20)
                .width(Fill)
                .class(theme::container::CardMuted),
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
