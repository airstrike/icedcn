//! A read-only view of a sale.
use iced::alignment::Horizontal::{Center, Right};
use iced::{Alignment, Element, Fill};
use icedcn::widget::scrollable::scrollable;
use icedcn::widget::table::Track::Auto;
use icedcn::widget::{button, column, container, row, space, table, text, tooltip};
use icedcn::{Slot, Theme, card, separator};

use super::super::{Action, Hotkey};
use super::{Instruction, Sale};
use crate::icon;

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    StartEdit,
}

pub fn view(sale: &Sale) -> Element<'_, Message, Theme> {
    let header = row![
        tooltip(
            button(icon::arrow_left().size(16))
                .style(button::Outline)
                .width(40)
                .on_press(Message::Back),
            "Back"
        ),
        text::large(&sale.name).semibold(),
        space::horizontal(),
        button("Edit")
            .style(button::Outline)
            .on_press(Message::StartEdit),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .width(Fill);
    let headings = table::header([table::row([
        table::head("Item Name"),
        table::head("Qty").align_x(Center).width(96),
        table::head("Price").align_x(Right).width(116),
        table::head("Tax Group").width(156),
        table::head("Total").align_x(Right).width(116),
    ])]);
    let body = table::body(sale.items.iter().map(|item| {
        table::row([
            table::cell(text::small(&item.name).medium()),
            table::cell(item.quantity().to_string()).align_x(Center),
            table::cell(format!("${:.2}", item.price())).align_x(Right),
            table::cell(text::muted(item.tax_group.to_string())),
            table::cell(text::small(format!("${:.2}", item.price() * item.quantity())).medium())
                .align_x(Right),
        ])
    }));
    let items = table([Slot::from(
        table::grid([Auto; 5]).header(headings).body(body),
    )]);
    let totals = column![
        row![
            text::muted("Subtotal"),
            space::horizontal(),
            text::small(format!("${:.2}", sale.calculate_subtotal()))
        ]
        .width(Fill),
        row![
            text::muted("Service Charge"),
            text::muted(format!("{}%", sale.service_charge_percent.unwrap_or(0.))),
            space::horizontal(),
            text::small(format!("${:.2}", sale.calculate_service_charge()))
        ]
        .spacing(12)
        .width(Fill),
        row![
            text::muted("Tax"),
            space::horizontal(),
            text::small(format!("${:.2}", sale.calculate_tax()))
        ]
        .width(Fill),
        row![
            text::muted("Gratuity"),
            space::horizontal(),
            text::small(format!("${:.2}", sale.gratuity_amount.unwrap_or(0.)))
        ]
        .width(Fill),
        separator(),
        row![
            text::label("Total"),
            space::horizontal(),
            text::large(format!("${:.2}", sale.calculate_total())).semibold()
        ]
        .align_y(Alignment::Center)
        .width(Fill),
    ]
    .spacing(8)
    .width(Fill);
    container(
        column![
            header,
            card(card::content(scrollable(items).height(Fill).width(Fill)).height(Fill))
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
        Hotkey::Escape => Action::instruction(Instruction::Back),
        _ => Action::none(),
    }
}
