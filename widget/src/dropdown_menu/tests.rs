use std::any::Any;

use iced_core::Length::Fill;
use iced_core::keyboard::key::Named;
use iced_core::time::Instant;
use iced_core::widget::operation::Focusable;
use iced_core::widget::{Id, Operation};
use iced_core::{Element, Event, Point, Rectangle, Size, window};
use iced_test::simulator;
use iced_widget::Renderer;
use icedcn_core::Theme;

use super::dropdown_menu;
use crate::input::input;
use crate::operation::focus;
use crate::{button, menu};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Search(String),
    Pick,
}

type Simulator = iced_test::Simulator<'static, Message, Theme, Renderer>;

fn view(open: bool) -> Element<'static, Message, Theme, Renderer> {
    dropdown_menu(
        |m| m.trigger(button("Chats")),
        |_| {
            crate::column![
                input("Search chats")
                    .id("search")
                    .on_input(Message::Search)
                    .width(Fill),
                menu::item("Coastal renewals").on_select(Message::Pick),
            ]
            .into()
        },
    )
    .default_open(open)
    .width(240)
    .into()
}

fn rebuild(ui: Simulator) -> Simulator {
    let mut ui = ui.rebuild(view(false));
    ui.draw(&Theme::light());
    ui
}

fn frame(ui: Simulator) -> Simulator {
    let mut ui = ui;
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    rebuild(ui)
}

/// Each focusable control: whether it has focus, and whether its ring shows.
#[derive(Default)]
struct Indicators(Vec<(bool, bool)>);

impl Operation for Indicators {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn focusable(&mut self, _: Option<&Id>, _: Rectangle, control: &mut dyn Focusable) {
        self.0.push((control.is_focused(), false));
    }

    fn custom(&mut self, _: Option<&Id>, _: Rectangle, state: &mut dyn Any) {
        if let Some(visible) = state.downcast_ref::<focus::Visible>()
            && let Some(last) = self.0.last_mut()
        {
            last.1 = visible.0;
        }
    }
}

fn indicators(ui: &mut Simulator) -> Vec<(bool, bool)> {
    let mut indicators = Indicators::default();
    ui.operate(&mut indicators);
    indicators.0
}

#[test]
fn a_focused_search_field_takes_typed_letters_before_row_typeahead() {
    let mut ui = Simulator::with_size(Default::default(), Size::new(640., 480.), view(true));
    ui.draw(&Theme::light());
    ui.click(Id::new("search"))
        .expect("the open menu shows its search");
    ui.typewrite("c");
    ui.typewrite(" ");
    let messages: Vec<_> = ui.into_messages().collect();
    assert!(
        messages.contains(&Message::Search("c".into())),
        "the search receives the letter a row also starts with: {messages:?}"
    );
    assert!(
        messages.contains(&Message::Search("c ".into())),
        "Space types into the search: {messages:?}"
    );
    assert!(
        !messages.contains(&Message::Pick),
        "no row was chosen: {messages:?}"
    );
}
