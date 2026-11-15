//! A read-only view of a sale.
use iced::{Alignment, Element, Fill};
use icedcn::widget::scrollable::scrollable;
use icedcn::widget::{button, column, container, row, space, text};
use icedcn::{Theme, theme};

use super::super::{Action, Hotkey};
use super::{Instruction, Sale};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    StartEdit,
}

pub fn view(sale: &Sale) -> Element<'_, Message, Theme> {
    let header = row![
        button(text("←").align_x(Alignment::Center))
            .width(40)
            .on_press(Message::Back),
        text(&sale.name).size(16),
        space::horizontal(),
        button("Edit").on_press(Message::StartEdit),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .width(Fill);
    let column_headers = row![
        text("Item Name").width(Fill),
        text("Qty").align_x(Alignment::Center).width(80),
        text("Price").align_x(Alignment::End).width(100),
        text("Tax Group").width(140),
        text("Total").align_x(Alignment::End).width(100),
    ]
    .spacing(2)
    .width(Fill);
    let items_list = sale.items.iter().fold(
        column![column_headers].spacing(5).width(Fill),
        |col, item| {
            col.push(
                container(
                    row![
                        text(&item.name).width(Fill),
                        text(item.quantity().to_string())
                            .align_x(Alignment::Center)
                            .width(80),
                        text(format!("${:.2}", item.price()))
                            .align_x(Alignment::End)
                            .width(100),
                        text(item.tax_group.to_string()).width(140),
                        text(format!("${:.2}", item.price() * item.quantity()))
                            .align_x(Alignment::End)
                            .width(100),
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
        .width(Fill),
        row![
            text("Service Charge").width(150),
            text(format!(
                "{}%",
                sale.service_charge_percent.map_or(0.0, |p| p)
            )),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_service_charge()))
        ]
        .width(Fill),
        row![
            text("Tax").width(150),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_tax()))
        ]
        .width(Fill),
        row![
            text("Gratuity").width(150),
            text(format!("${:.2}", sale.gratuity_amount.unwrap_or(0.0))),
            space::horizontal(),
            text(format!("${:.2}", sale.gratuity_amount.unwrap_or(0.0)))
        ]
        .width(Fill),
        row![
            text("Total").width(150).size(16),
            space::horizontal(),
            text(format!("${:.2}", sale.calculate_total())).size(16)
        ]
        .width(Fill),
    ]
    .spacing(2)
    .width(Fill);
    container(
        column![
            header,
            container(
                scrollable(column![items_list].spacing(10).padding(20).width(Fill))
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
        Hotkey::Escape => Action::instruction(Instruction::Back),
        _ => Action::none(),
    }
}
