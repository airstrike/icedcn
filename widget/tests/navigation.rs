//! Keyboard navigation through compound controls and modal focus scopes.
use iced_core::keyboard::key::Named;
use iced_core::time::Instant;
use iced_core::widget::operation::{self, Operation, Outcome, focusable};
use iced_core::{Element, Event, Point, window};
use iced_test::simulator::{self, simulator};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_widget::{accordion, button, column, dialog};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Open(Vec<String>),
    Pressed,
}
type Simulator<'a> = iced_test::Simulator<'a, Message, Theme, Renderer>;

fn focus(ui: &mut Simulator<'_>) -> focusable::Count {
    let mut count = focusable::count();
    ui.operate(&mut operation::black_box(&mut count));
    let Outcome::Some(count) = count.finish() else {
        panic!("focus count")
    };
    count
}

fn frame(ui: &mut Simulator<'_>) {
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
}

#[test]
fn accordion_arrows_skip_disabled_headers_and_activate_the_selected_item() {
    let content: Element<'_, Message, Theme, Renderer> = accordion([
        accordion::item("one", "One"),
        accordion::item("two", "Two").disabled(true),
        accordion::item("three", "Three").content(button("Child")),
    ])
    .on_value_change(Message::Open)
    .into();
    let mut ui = simulator(content);
    assert_eq!(
        focus(&mut ui).total,
        2,
        "closed panels are outside the focus order"
    );
    let _ = ui.simulate([simulator::press_key(Named::Tab, None)]);
    // The host's Tab handler uses the iced operation; simulate its two passes.
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut step) = next.finish() {
        ui.operate(step.as_mut());
    }
    ui.tap_key(Named::ArrowDown);
    assert_eq!(focus(&mut ui).focused, Some(1));
    ui.tap_key(Named::Space);
    assert_eq!(focus(&mut ui).total, 3);
    ui.tap_key(Named::Home);
    assert_eq!(focus(&mut ui).focused, Some(0));
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(vec!["three".into()])]
    );
}

#[test]
fn dialog_focus_enters_wraps_and_returns_on_escape() {
    let body = || -> Element<'static, Message, Theme, Renderer> {
        dialog(
            |d| d.trigger(button("Open")),
            |_| column![button("One").on_press(Message::Pressed), button("Two")].into(),
        )
        .show_close_button(false)
        .into()
    };
    let mut ui = simulator(body());
    ui.point_at(Point::new(10.0, 10.0));
    let _ = ui.simulate(simulator::click());
    frame(&mut ui);
    assert_eq!(
        focus(&mut ui),
        focusable::Count {
            focused: Some(0),
            total: 2
        }
    );
    ui.tap_key(Named::Tab);
    assert_eq!(focus(&mut ui).focused, Some(1));
    ui.tap_key(Named::Tab);
    assert_eq!(focus(&mut ui).focused, Some(0));
    ui.tap_key(Named::Enter);
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(body());
    frame(&mut ui);
    assert_eq!(
        focus(&mut ui),
        focusable::Count {
            focused: Some(0),
            total: 1
        }
    );
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [Message::Pressed]);
}
