//! Linked badges participate in keyboard focus traversal and activation.
use iced_core::keyboard::key::Named;
use iced_core::widget::Operation;
use iced_core::widget::operation::{Outcome, focusable};
use iced_core::{Element, Event, Point, mouse};
use iced_test::simulator::{self, simulator};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_widget::badge;

#[test]
fn keyboard_activates_link_without_hover() {
    let element: Element<'_, _, Theme, Renderer> =
        badge("Open Link").icon_end().on_press(true).into();
    let mut ui = simulator(element);
    let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(600.0, 300.0),
    })]);
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut second) = next.finish() {
        ui.operate(second.as_mut());
    }
    let _ = ui.simulate([simulator::press_key(Named::Enter, None)]);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [true]);
}
