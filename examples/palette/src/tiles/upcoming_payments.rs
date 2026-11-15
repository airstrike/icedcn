//! Select a date to inspect the recurring sample payment schedule.

use iced::widget::component::{Component, component};
use iced::{Element, Fill, Widget};
use icedcn::widget::calendar::{Datelike, Selection};
use icedcn::widget::{calendar, column, container, separator, text};
use icedcn::{Theme, card};

use crate::Message;

struct Preview;

impl<'a> Component<'a, Message, Theme> for Preview {
    type State = Option<Selection>;
    type Event = Option<Selection>;

    fn update(
        &self,
        state: &mut Self::State,
        event: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = event;
        None
    }

    fn view(&self, selected: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let detail = selected.map_or_else(
            || "Select a date to see scheduled payments.".to_owned(),
            |selection| {
                let payment = match selection.start.day() {
                    1 => "Rent · $1,250.00",
                    15 => "Internet · $65.00",
                    25 => "Studio subscription · $24.00",
                    _ => "No payments scheduled",
                };
                format!("{}\n{payment}", selection.start.format("%B %-d"))
            },
        );
        Element::from(
            card(
                column![
                    card::header("Upcoming Payments").description("Monthly demo schedule"),
                    card::content(
                        column![
                            container(
                                calendar()
                                    .selected(*selected)
                                    .on_select(|value| value)
                                    .caption_dropdown(true)
                            )
                            .center_x(Fill),
                            separator(),
                            text::small(detail),
                            text::muted("Rent · 1st / Internet · 15th / Studio · 25th"),
                        ]
                        .spacing(12)
                    ),
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
