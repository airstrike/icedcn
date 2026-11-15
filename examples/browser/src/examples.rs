//! Runnable applications assembled from icedcn components.
pub mod receipts;

use iced::keyboard::Modifiers;
use iced::widget::operation;
use iced::{Element, Fill, Task, Widget};
use icedcn::widget::{button, column, container, row, scrollable, text};
use icedcn::{Theme, card};

use crate::Message;

#[derive(Default)]
pub struct State {
    receipts: receipts::State,
    open: bool,
}

#[derive(Debug, Clone)]
pub enum Event {
    Gallery,
    OpenReceipts,
    Receipts(receipts::Message),
}

impl State {
    pub fn update(&mut self, event: Event) -> Task<Message> {
        match event {
            Event::Gallery => self.open = false,
            Event::OpenReceipts => self.open = true,
            Event::Receipts(message) => {
                return self
                    .receipts
                    .update(message)
                    .map(|m| Message::Example(Event::Receipts(m)));
            }
        }
        Task::none()
    }

    pub fn escape(&mut self) -> Task<Message> {
        if self.open {
            self.update(Event::Receipts(receipts::Message::Hotkey(
                receipts::Hotkey::Escape,
            )))
        } else {
            Task::none()
        }
    }

    pub fn focus(&mut self, previous: bool) -> Task<Message> {
        if self.open {
            let modifiers = if previous {
                Modifiers::SHIFT
            } else {
                Modifiers::empty()
            };
            self.update(Event::Receipts(receipts::Message::Hotkey(
                receipts::Hotkey::Tab(modifiers),
            )))
        } else if previous {
            operation::focus_previous()
        } else {
            operation::focus_next()
        }
    }

    pub fn view(&self, width: f32) -> Element<'_, Message, Theme> {
        if self.open {
            return column![
                container(
                    button("← All examples")
                        .style(button::Ghost)
                        .on_press(Message::Example(Event::Gallery))
                )
                .padding([8, 20]),
                self.receipts
                    .view()
                    .map(|message| Message::Example(Event::Receipts(message)))
                    .boxed(),
            ]
            .width(Fill)
            .height(Fill)
            .into();
        }
        let content = column![
            text::h1("Examples"),
            card(column![
                card::header("Receipts").description("A sales notebook."),
                card::footer(row![
                    button("Open Receipts").on_press(Message::Example(Event::OpenReceipts)),
                    button("View source").style(button::Outline)
                        .on_press(Message::OpenUrl("https://github.com/airstrike/icedcn/tree/HEAD/examples/browser/src/examples")),
                ].spacing(8).width(Fill)),
            ].width(Fill)).width(Fill),
        ].spacing(24).width(Fill).max_width(1100.);
        scrollable::scrollable(
            container(content)
                .padding(if width < 600. { 16 } else { 32 })
                .center_x(Fill),
        )
        .height(Fill)
        .width(Fill)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_test::core::Settings;
    use iced_test::selector::Candidate;
    use iced_test::simulator::Simulator;

    #[test]
    fn gallery_centers_its_capped_content_and_fills_the_card_footer() {
        let state = State::default();
        for width in [375., 1440.] {
            let mut ui = Simulator::with_size(
                Settings::default(),
                iced::Size::new(width, 1200.),
                state.view(width),
            );
            let expected = (width - if width < 600. { 32. } else { 64. }).min(1100.);
            let heading = ui.find("Examples").unwrap().bounds();
            assert!(
                (heading.x - (width - expected) / 2.).abs() < 1.,
                "heading at {heading:?}"
            );
            let open = ui.find("Open Receipts").unwrap().bounds();
            let mut footer = false;
            let _ = ui.find(|candidate: Candidate<'_>| {
                if let Candidate::Container { bounds, .. } = candidate {
                    footer |= bounds.contains(open.center())
                        && bounds.height < 100.
                        && (bounds.width - expected).abs() < 1.;
                }
                None::<()>
            });
            assert!(footer, "the footer surface must span the {expected}px card");
        }
    }
}
