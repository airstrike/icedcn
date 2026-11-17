//! Each thumb owns its focus and cannot pass the neighboring values.
use iced_core::keyboard::key::Named;
use iced_core::time::{Duration, Instant};
use iced_core::{Event, Length, Point, Settings, Size, Widget, mouse, touch, window};
use iced_test::Simulator;
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, column, scrollable, space};
use icedcn_core::{Theme, Vertical};
use icedcn_widget::slider::{self, Slider};

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

/// A vertical slider at 50 at the bottom of a page scrolled to its end, where
/// event positions (window space) and the cursor (content space) differ.
fn scrolled_page() -> Simulator<'static, u8, Theme, Renderer> {
    let mut ui = Simulator::with_size(
        Settings::default(),
        (100.0, 200.0),
        scrollable(column![
            space().height(300),
            slider::slider(0..=100)
                .orientation(Vertical)
                .height(116)
                .default_value(50)
                .on_value_change(|v| v),
        ])
        .width(100),
    );
    ui.point_at(Point::new(8.0, 100.0));
    let _ = ui.scroll(mouse::ScrollDelta::Pixels { x: 0.0, y: -1000.0 });
    let now = Instant::now();
    for frame in 0..60 {
        let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(
            now + Duration::from_millis(frame * 50),
        ))]);
    }
    // The track now spans window y 84..200; its thumb sits at y 142.
    ui
}

#[test]
fn vertical_drag_follows_the_mouse_in_a_scrolled_page() {
    let mut ui = scrolled_page();
    ui.point_at(Point::new(8.0, 142.0));
    let _ = ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(Point::new(8.0, 94.0));
    let _ = ui.simulate([
        Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(8.0, 94.0),
        }),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    ]);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [98]);
}

#[test]
fn vertical_drag_follows_a_finger_in_a_scrolled_page() {
    let mut ui = scrolled_page();
    let id = touch::Finger(1);
    for (event, position) in [
        (
            touch::Event::FingerPressed {
                id,
                position: Point::new(8.0, 180.0),
                timestamp: None,
            },
            Point::new(8.0, 180.0),
        ),
        (
            touch::Event::FingerMoved {
                id,
                position: Point::new(8.0, 94.0),
                timestamp: None,
            },
            Point::new(8.0, 94.0),
        ),
        (
            touch::Event::FingerLifted {
                id,
                position: Point::new(8.0, 94.0),
                timestamp: None,
            },
            Point::new(8.0, 94.0),
        ),
    ] {
        ui.point_at(position);
        let _ = ui.simulate([Event::Touch(event)]);
    }
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [12, 98]);
}

#[test]
fn vertical_size_does_not_depend_on_builder_order() {
    let size = |s: Slider<'_, f32, (), Theme>| Widget::<(), Theme, Renderer>::size(&s);
    let before = size(slider::slider(0.0..=1.0).height(120).orientation(Vertical));
    let after = size(slider::slider(0.0..=1.0).orientation(Vertical).height(120));
    assert_eq!(before, after);
    assert_eq!(before, Size::new(Length::from(16), Length::from(120)));
    assert_eq!(
        size(slider::slider(0.0..=1.0).orientation(Vertical)),
        Size::new(Length::from(16), Length::from(160))
    );
}
