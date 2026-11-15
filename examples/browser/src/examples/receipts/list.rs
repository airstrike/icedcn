//! List sales and navigate to sale details or editing.
use std::collections::HashMap;

use iced::alignment::Horizontal::Right;
use iced::{Center, Element, Fill, Shrink};
use icedcn::widget::scrollable::scrollable;
use icedcn::widget::table::Track::Auto;
use icedcn::widget::{button, column, container, empty, row, space, table, text};
use icedcn::{Slot, Theme, card};

use super::sale::Sale;
use crate::icon;

#[derive(Debug, Clone)]
pub enum Message {
    NewSale,
    SelectSale(usize),
}

pub fn view(sales: &HashMap<usize, Sale>) -> Element<'_, Message, Theme> {
    let main_content: Element<'_, Message, Theme> = if sales.is_empty() {
        container(
            empty()
                .media(empty::media(icon::file_text().size(24)).style(empty::Icon))
                .content(button("Create your first sale →").on_press(Message::NewSale)),
        )
        .center(Fill)
        .into()
    } else {
        let header = table::header([table::row([
            table::head("Sale"),
            table::head("Total").align_x(Right).width(140),
        ])]);
        let body = table::body(sales.iter().map(|(id, sale)| {
            table::row([table::cell(
                button(
                    row![
                        text::small(&sale.name).medium().width(Fill),
                        text::small(format!("${:.2}", sale.calculate_total())).medium(),
                    ]
                    .spacing(16)
                    .align_y(Center)
                    .width(Fill),
                )
                .style(button::Ghost)
                .translate(false)
                .height(Shrink)
                .padding([12, 8])
                .width(Fill)
                .on_press(Message::SelectSale(*id)),
            )
            .span(2)
            .padding(0)])
        }));
        let sales_table = table([Slot::from(table::grid([Auto; 2]).header(header).body(body))]);
        column![
            row![
                space::horizontal(),
                button("New Sale").on_press(Message::NewSale)
            ]
            .align_y(Center)
            .width(Fill),
            card(card::content(scrollable(sales_table).width(Fill).height(Fill)).height(Fill))
                .width(Fill)
                .height(Fill),
        ]
        .spacing(20)
        .width(Fill)
        .height(Fill)
        .into()
    };
    container(main_content)
        .padding(20)
        .id(super::ROOT)
        .width(Fill)
        .height(Fill)
        .into()
}
