//! Runnable applications assembled from icedcn components.
pub mod playground;
pub mod receipts;
mod window;

use iced::keyboard::Modifiers;
use iced::widget::operation;
use iced::{Element, Fill, Task, Widget};
use icedcn::widget::{button, column, container, row, scrollable, text};
use icedcn::{Theme, card};

use crate::Message;

#[derive(Default)]
pub struct State {
    receipts: receipts::State,
    playground: playground::State,
    open: Option<Open>,
    platform: window::Platform,
}

/// The example on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Open {
    Receipts,
    Playground,
}

#[derive(Debug, Clone)]
pub enum Event {
    Gallery,
    OpenReceipts,
    Receipts(receipts::Message),
    OpenPlayground,
    Playground(playground::Message),
}

impl State {
    pub(crate) fn is_open(&self) -> bool {
        self.open.is_some()
    }

    /// Whether the example on screen is the playground.
    pub(crate) fn is_playground(&self) -> bool {
        self.open == Some(Open::Playground)
    }

    pub fn update(&mut self, event: Event) -> Task<Message> {
        match event {
            Event::Gallery => self.open = None,
            Event::OpenReceipts => self.open = Some(Open::Receipts),
            Event::Receipts(message) => {
                return self
                    .receipts
                    .update(message)
                    .map(|m| Message::Example(Event::Receipts(m)));
            }
            Event::OpenPlayground => self.open = Some(Open::Playground),
            Event::Playground(message) => {
                return self
                    .playground
                    .update(message)
                    .map(|m| Message::Example(Event::Playground(m)));
            }
        }
        Task::none()
    }

    pub fn escape(&mut self) -> Task<Message> {
        if self.open == Some(Open::Receipts) {
            self.update(Event::Receipts(receipts::Message::Hotkey(
                receipts::Hotkey::Escape,
            )))
        } else {
            Task::none()
        }
    }

    pub fn focus(&mut self, previous: bool) -> Task<Message> {
        if self.open == Some(Open::Receipts) {
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
        if let Some(open) = self.open {
            let (title, example) = match open {
                Open::Receipts => (
                    self.receipts.title(),
                    self.receipts
                        .view()
                        .map(|message| Message::Example(Event::Receipts(message)))
                        .boxed(),
                ),
                Open::Playground => (
                    self.playground.title(),
                    self.playground
                        .view(width)
                        .map(|message| Message::Example(Event::Playground(message)))
                        .boxed(),
                ),
            };
            return column![
                container(
                    button("← All examples")
                        .anchor()
                        .style(button::Ghost)
                        .on_press(Message::Example(Event::Gallery))
                )
                .padding([8, 20]),
                self.platform.view(width, title, example),
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
                    button("Open Receipts").anchor().on_press(Message::Example(Event::OpenReceipts)),
                    button("View source").anchor().style(button::Outline)
                        .on_press(Message::OpenUrl("https://github.com/airstrike/icedcn/tree/HEAD/examples/browser/src/examples")),
                ].spacing(8).width(Fill)),
            ].width(Fill)).width(Fill),
            card(column![
                card::header("Playground").description("A Rust scratchpad."),
                card::footer(row![
                    button("Open Playground").anchor().on_press(Message::Example(Event::OpenPlayground)),
                    button("View source").anchor().style(button::Outline)
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
    use iced_test::core::widget::Operation;
    use iced_test::core::widget::operation::TextInput;
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

    #[test]
    fn playground_card_opens_and_prints_the_starter_program() {
        let mut state = State::default();
        let mut ui = Simulator::with_size(
            Settings::default(),
            iced::Size::new(1440., 1200.),
            state.view(1440.),
        );
        ui.click("Open Playground").unwrap();
        let Some(Message::Example(event)) = ui.into_messages().next() else {
            panic!("the playground card must publish its open event");
        };
        assert!(matches!(event, Event::OpenPlayground));
        let _ = state.update(event);
        assert!(state.is_playground());
        let printed = state
            .playground
            .printed()
            .expect("the starter program must run")
            .to_owned();
        let first = printed.lines().next().unwrap();
        assert_eq!(first, "$  12.50  tip $  2.25  (18.0%)");
        for (width, side_by_side) in [(1440., true), (390., false)] {
            let mut ui = Simulator::with_size(
                Settings::default(),
                iced::Size::new(width, 1200.),
                state.view(width),
            );
            let mut editors = Editors::default();
            ui.operate(&mut editors);
            let [(editor, source)] = editors.0.as_slice() else {
                panic!("the playground has one editor");
            };
            assert!(source.starts_with("// Each bill"));
            ui.find("main.rs").unwrap();
            let output = ui.find("Output").unwrap().bounds();
            ui.find(printed.as_str()).unwrap();
            if side_by_side {
                assert!(
                    output.x > editor.x + editor.width,
                    "{output:?} beside {editor:?}"
                );
                assert!(output.y < editor.y + editor.height);
            } else {
                assert!(
                    output.y > editor.y + editor.height,
                    "{output:?} below {editor:?}"
                );
            }
        }
    }

    #[derive(Default)]
    struct Editors(Vec<(iced::Rectangle, String)>);
    impl Operation for Editors {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn text_input(
            &mut self,
            _: Option<&iced::widget::Id>,
            bounds: iced::Rectangle,
            input: &mut dyn TextInput,
        ) {
            self.0.push((bounds, input.text().into_owned()));
        }
    }

    #[test]
    fn framed_receipts_updates_title_and_keeps_gallery_navigation_outside() {
        use iced::widget::Id;

        let mut state = State {
            open: Some(Open::Receipts),
            platform: window::Platform::Mac,
            ..State::default()
        };
        let mut ui = Simulator::with_size(
            Settings::default(),
            iced::Size::new(1440., 900.),
            state.view(1440.),
        );
        let frame = ui.find(Id::from("example-window")).unwrap().bounds();
        let back = ui.find("← All examples").unwrap().bounds();
        assert!(back.y + back.height <= frame.y);
        ui.find("iced Receipts").unwrap();
        ui.click("Create your first sale →").unwrap();
        let Message::Example(event) = ui.into_messages().next().unwrap() else {
            panic!("the receipt action must retain its message mapping");
        };
        let _ = state.update(event);
        let mut ui = Simulator::with_size(
            Settings::default(),
            iced::Size::new(1440., 900.),
            state.view(1440.),
        );
        ui.find(state.receipts.title()).unwrap();
        ui.find("Save").unwrap();
        ui.find(Id::from("receipts-sale-name")).unwrap();
        ui.click("← All examples").unwrap();
        assert!(matches!(
            ui.into_messages().next(),
            Some(Message::Example(Event::Gallery))
        ));
    }
}
