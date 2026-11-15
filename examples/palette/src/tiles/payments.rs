//! Sample payments with per-row actions and a local detail view.

use iced::widget::component::{Component, component};
use iced::{Center, Element, Fill, Widget};
use icedcn::widget::{
    badge, button, column, dropdown_menu, menu, menu_group, row, separator, space, text,
};
use icedcn::{Size, Theme, card};

use crate::{Message, icon};

const PAYMENTS: &[(&str, &str)] = &[
    ("Studio rental", "$240.00"),
    ("Mastering session", "$180.00"),
    ("Distribution", "$24.00"),
];

struct Preview;

#[derive(Default)]
struct Model {
    paid: [bool; 3],
    detail: Option<usize>,
}

#[derive(Debug, Clone)]
enum Event {
    MarkPaid(usize),
    Details(usize),
    Close,
}

impl<'a> Component<'a, Message, Theme> for Preview {
    type State = Model;
    type Event = Event;

    fn update(&self, state: &mut Model, event: Event, _: &iced::Renderer) -> Option<Message> {
        match event {
            Event::MarkPaid(index) => state.paid[index] = true,
            Event::Details(index) => state.detail = Some(index),
            Event::Close => state.detail = None,
        }
        None
    }

    fn view(&self, state: &Model) -> impl Widget<Event, Theme> + 'a {
        let mut rows = column![].spacing(12).width(Fill);
        for (index, (name, amount)) in PAYMENTS.iter().enumerate() {
            let paid = state.paid[index];
            rows = rows
                .push(
                    row![
                        column![text::small(*name).semibold(), text::muted(*amount)].spacing(2),
                        space::horizontal(),
                        badge(if paid { "Paid" } else { "Pending" }).style(badge::Secondary),
                        dropdown_menu(
                            |m| m.trigger(
                                button(icon::ellipsis().size(16))
                                    .style(button::Ghost)
                                    .size(Size::Icon)
                            ),
                            move |_| menu_group![
                                menu::item("View details").on_select(Event::Details(index)),
                                menu::item("Mark as paid")
                                    .on_select(Event::MarkPaid(index))
                                    .disabled(paid),
                            ],
                        )
                        .width(160),
                    ]
                    .spacing(8)
                    .align_y(Center)
                    .into(),
                )
                .push(separator().into());
        }
        if let Some(index) = state.detail {
            let (name, amount) = PAYMENTS[index];
            rows = rows.push(
                column![
                    text::small(format!("{name} · {amount}")),
                    text::muted("Sample invoice · Bank transfer · Due Nov 25"),
                    button("Close details")
                        .style(button::Outline)
                        .size(Size::Sm)
                        .on_press(Event::Close),
                ]
                .spacing(8)
                .into(),
            );
        }
        Element::from(
            card(
                column![
                    card::header("Payments").description("Review and organize sample invoices."),
                    card::content(rows),
                ]
                .spacing(16),
            )
            .width(Fill),
        )
    }
}

pub fn view(_: &crate::State) -> Element<'static, Message, Theme> {
    component(Preview).boxed()
}
