//! A divider holds focus across a rebuild, including after a pointer drag.
use iced_core::keyboard::key::Named;
use iced_core::{Element, Event, Point, mouse};
use iced_test::Simulator;
use iced_test::simulator::{self, simulator};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_widget::resizable;

#[test]
fn dragging_and_keyboard_adjust_the_same_split() {
    let view = || {
        Element::from(
            resizable("Left", "Right")
                .width(201)
                .height(80)
                .on_value_change(|v| v),
        )
    };
    let mut ui: Simulator<'_, _, Theme, Renderer> = simulator(view());
    ui.point_at(Point::new(100.0, 40.0));
    let _ = ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    let p = Point::new(150.0, 40.0);
    ui.point_at(p);
    let _ = ui.simulate([
        Event::Mouse(mouse::Event::CursorMoved { position: p }),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    ]);
    let mut ui = ui.rebuild(view());
    let _ = ui.simulate([
        simulator::press_key(Named::ArrowLeft, None),
        simulator::press_key(Named::Home, None),
    ]);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [0.75, 0.74, 0.0]);
}
