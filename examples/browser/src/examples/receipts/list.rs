//! List sales and navigate to sale details or editing.
use std::collections::HashMap;

use iced::{Center, Element, Fill, Shrink};
use icedcn::widget::{button, column, container, row, space, text};
use icedcn::{Theme, theme};

use super::sale::Sale;

#[derive(Debug, Clone)]
pub enum Message {
    NewSale,
    SelectSale(usize),
}

pub fn view(sales: &HashMap<usize, Sale>) -> Element<'_, Message, Theme> {
    let main_content: Element<'_, Message, Theme> = if sales.is_empty() {
        container(button("Create your first sale →").on_press(Message::NewSale))
            .center(Fill)
            .into()
    } else {
        let mut sales_list = column![].spacing(10).width(Fill);
        for (id, sale) in sales {
            sales_list = sales_list.push(
                button(
                    row![
                        column![
                            text(&sale.name).size(13),
                            text(format!("Total: ${:.2}", sale.calculate_total()))
                                .size(12)
                                .style(theme::text::Muted),
                        ]
                        .width(Fill)
                        .padding(10),
                    ]
                    .width(Fill),
                )
                .style(button::Secondary)
                .height(Shrink)
                .on_press(Message::SelectSale(*id))
                .width(Fill)
                .into(),
            );
        }
        column![
            row![
                space::horizontal(),
                button(text("New Sale").size(14)).on_press(Message::NewSale),
            ]
            .align_y(Center)
            .width(Fill),
            sales_list,
        ]
        .spacing(20)
        .width(Fill)
        .into()
    };
    container(column![main_content].spacing(20).width(Fill).height(Fill))
        .padding(20)
        .id(super::ROOT)
        .width(Fill)
        .height(Fill)
        .into()
}
