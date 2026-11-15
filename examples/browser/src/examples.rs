//! Runnable applications assembled from icedcn components.
pub mod receipts;

use iced::{Element, Fill, Task, Widget};
use icedcn::widget::{badge, button, column, container, row, scrollable, text};
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
            self.update(Event::Receipts(receipts::Message::Back))
        } else {
            Task::none()
        }
    }

    pub fn view(&self, width: f32) -> Element<'_, Message, Theme> {
        let content = if self.open {
            column![
                button("← All examples")
                    .style(button::Ghost)
                    .on_press(Message::Example(Event::Gallery)),
                self.receipts
                    .view(width)
                    .map(|m| Message::Example(Event::Receipts(m)))
                    .boxed(),
            ]
            .spacing(24)
            .width(Fill)
        } else {
            column![
                text::h1("Examples"),
                text::lead("Real applications built with icedcn."),
                text::muted("Open an example and try the complete workflow. Each runs here on desktop and in your browser."),
                card(column![
                    card::header("Receipts")
                        .description("A sales notebook with a list, receipt details, and an editable line-item form."),
                    card::content(column![
                        row![badge("Forms"), badge("Tables").style(badge::Secondary), badge("Navigation").style(badge::Outline)].spacing(8),
                        text::p("Create a sale, add items, choose tax groups, and calculate service charges and gratuity. Edit a saved receipt or cancel your draft."),
                        text::muted("Sample receipts are included. Changes stay in this session."),
                    ].spacing(16)),
                    card::footer(row![
                        button("Open Receipts").on_press(Message::Example(Event::OpenReceipts)),
                        button("View source").style(button::Outline).on_press(Message::OpenUrl("https://github.com/airstrike/icedcn/tree/HEAD/examples/browser/src/examples")),
                    ].spacing(8)),
                ]).width(Fill),
            ]
            .spacing(24)
            .width(Fill)
        };
        scrollable::scrollable(
            container(content)
                .max_width(1100)
                .padding(if width < 600. { 16 } else { 32 })
                .center_x(Fill),
        )
        .height(Fill)
        .width(Fill)
        .into()
    }
}
