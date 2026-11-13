//! Each thumb owns its focus and cannot pass the neighboring values.
use iced_core::keyboard::key::Named;
use iced_core::{Event, Point, mouse};
use iced_test::Simulator;
use iced_test::simulator::{self, simulator};
use iced_widget::Renderer;
use icedxdy_core::{Theme, Vertical};
use icedxdy_widget::slider;

#[test]
fn keyboard_clamps_one_thumb_and_preserves_controlled_values() {
    let mut ui: Simulator<'_, _, Theme, Renderer> = simulator(
        slider::range(0..=100)
            .width(116)
            .value([25, 75])
            .on_value_change(|v| v),
    );
    ui.point_at(Point::new(33.0, 8.0));
    let _ = ui.simulate(simulator::click());
    ui.point_at(Point::new(200.0, 100.0));
    let _ = ui.simulate([
        simulator::press_key(Named::End, None),
        simulator::press_key(Named::ArrowRight, None),
    ]);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [[75, 75], [26, 75]]);
}

#[test]
fn vertical_drag_respects_neighbors_after_a_rebuild() {
    let view = || {
        slider::multiple(0..=100, [10, 20, 70])
            .orientation(Vertical)
            .height(116)
            .step(10)
            .on_value_change(|v| v)
    };
    let mut ui: Simulator<'_, _, Theme, Renderer> = simulator(view());
    ui.point_at(Point::new(8.0, 88.0));
    let _ = ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(Point::new(8.0, 8.0));
    let _ = ui.simulate([
        Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(8.0, 8.0),
        }),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    ]);
    let mut ui = ui.rebuild(view());
    let _ = ui.simulate([simulator::press_key(Named::Home, None)]);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [[10, 70, 70], [10, 10, 70]]
    );
}
