use browser::component::tooltip::TooltipPage;
use iced::keyboard::key::Named;
use iced::touch::{self, Finger};
use iced::{Element, Event, Fill, Point, Size, mouse};
use iced_test::core::widget::operation::{Operation, Outcome, focusable};
use iced_test::{Simulator, simulator};
use icedcn::Theme;
use icedcn::widget::{button, container, tooltip};

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Open(bool),
    Activate,
}

fn frame<M: 'static>(ui: &mut Simulator<'_, M, Theme>) {
    ui.draw(&Theme::light());
}

fn move_to<M: 'static>(ui: &mut Simulator<'_, M, Theme>, position: Point) {
    ui.point_at(position);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position })]);
    frame(ui);
}

fn next<M: 'static>(ui: &mut Simulator<'_, M, Theme>) {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut step) = op.finish() {
        ui.operate(step.as_mut());
    }
    frame(ui);
}

fn hint() -> Element<'static, Message, Theme> {
    container(
        tooltip(button("Help").on_press(Message::Activate), "Hint").on_open_change(Message::Open),
    )
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

#[test]
fn pressing_the_trigger_dismisses_the_hint_and_delivers_its_action() {
    let mut ui = Simulator::new(hint());
    let point = ui.find("Help").unwrap().bounds().center();
    move_to(&mut ui, point);
    ui.find("Hint").unwrap();
    ui.simulate(simulator::click());
    frame(&mut ui);
    assert!(ui.find("Hint").is_err());
    let messages: Vec<_> = ui.drain().collect();
    assert!(messages.contains(&Message::Activate));
    assert!(messages.contains(&Message::Open(false)));
    move_to(&mut ui, point);
    assert!(ui.find("Hint").is_err());
    move_to(&mut ui, Point::ORIGIN);
    move_to(&mut ui, point);
    ui.find("Hint").unwrap();
}

#[test]
fn touch_tap_activates_without_opening_a_hover_hint() {
    let mut ui = Simulator::new(hint());
    let position = ui.find("Help").unwrap().bounds().center();
    ui.point_at(position);
    ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            id: Finger(2),
            position,
        }),
        Event::Touch(touch::Event::FingerLifted {
            id: Finger(2),
            position,
        }),
    ]);
    frame(&mut ui);
    assert!(ui.find("Hint").is_err());
    assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Activate]);
}

#[test]
fn an_initially_open_hint_dismisses_and_a_controlled_hint_requests_dismissal() {
    for controlled in [false, true] {
        let view = |open| -> Element<'_, Message, Theme> {
            let tip = tooltip(button("Help"), "Hint").on_open_change(Message::Open);
            let tip = if controlled {
                tip.open(open)
            } else {
                tip.default_open(open)
            };
            container(tip).center_x(Fill).center_y(Fill).into()
        };
        let mut ui = Simulator::new(view(true));
        frame(&mut ui);
        ui.find("Hint").unwrap();
        ui.tap_key(Named::Escape);
        frame(&mut ui);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Open(false)]);
        if controlled {
            ui.find("Hint").unwrap();
            ui = ui.rebuild(view(false));
            frame(&mut ui);
        }
        assert!(ui.find("Hint").is_err());
    }
}

#[test]
fn registered_side_buttons_wrap_and_keyboard_focus_follows_their_display_order() {
    let page = TooltipPage::new(0);
    let mut ui = Simulator::with_size(
        Default::default(),
        Size::new(142., 160.),
        page.preview("sides"),
    );
    let first = ui.find("Left").unwrap().bounds();
    let last = ui.find("Right").unwrap().bounds();
    assert!(last.y > first.y && last.x + last.width <= 142.);
    for label in ["Left", "Top", "Bottom", "Right"] {
        next(&mut ui);
        let trigger = ui.find(label).unwrap().bounds();
        let hint = ui.find("Add to library").unwrap().bounds();
        assert!(hint.x >= 0. && hint.x + hint.width <= 142.);
        assert!(
            hint.y + hint.height <= trigger.y
                || hint.y >= trigger.y + trigger.height
                || hint.x + hint.width <= trigger.x
                || hint.x >= trigger.x + trigger.width
        );
    }
    ui.tap_key(Named::Escape);
    frame(&mut ui);
    assert!(ui.find("Add to library").is_err());
}

#[test]
fn disabled_button_has_a_pointer_hint_and_no_tab_stop() {
    let page = TooltipPage::new(0);
    let mut ui = Simulator::new(page.preview("disabled"));
    next(&mut ui);
    assert!(ui.find("This feature is currently unavailable").is_err());
    let center = ui.find("Disabled").unwrap().bounds().center();
    move_to(&mut ui, center);
    ui.find("This feature is currently unavailable").unwrap();
}
