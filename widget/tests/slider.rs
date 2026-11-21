//! Each thumb owns its focus and cannot pass the neighboring values.
use iced_core::keyboard::key::Named;
use iced_core::time::{Duration, Instant};
use iced_core::{Event, Length, Point, Settings, Size, Widget, mouse, touch, window};
use iced_test::Simulator;
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, column, scrollable, space};
use icedcn_core::{Horizontal, Orientation, Theme, Vertical};
use icedcn_widget::slider::{self, Slider};

#[test]
fn keyboard_clamps_one_thumb_and_preserves_controlled_values() {
    let mut ui: Simulator<'_, _, Theme, Renderer> = simulator(
        slider::range(0..=100)
            .width(116)
            .value([25, 75])
            .on_value_change(|v| v),
    );
    ui.draw(&Theme::default());
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
    ui.draw(&Theme::default());
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
    ui.draw(&Theme::default());
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
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [96]);
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
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [13, 96]);
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

/// A 116px track: 12px nova thumbs centre on `6 + value * 1.04` from the
/// start edge, like Radix's `getThumbInBoundsOffset`.
const LENGTH: f32 = 116.0;

fn center(value: u8) -> f32 {
    6.0 + f32::from(value) * 1.04
}

fn at(orientation: Orientation, offset: f32) -> Point {
    match orientation {
        Horizontal => Point::new(offset, 8.0),
        Vertical => Point::new(8.0, LENGTH - offset),
    }
}

fn drawn<'a, Values: slider::Thumbs<u8> + PartialEq + std::fmt::Debug + 'static>(
    slider: Slider<'a, u8, Values, Theme, Values>,
    orientation: Orientation,
) -> Simulator<'a, Values, Theme, Renderer> {
    let slider = slider.orientation(orientation);
    let slider = match orientation {
        Horizontal => slider.width(LENGTH),
        Vertical => slider.height(LENGTH),
    };
    let mut ui = simulator(slider.on_value_change(|v| v));
    ui.draw(&Theme::default());
    ui
}

fn press<'a, M: 'a>(ui: &mut Simulator<'a, M, Theme, Renderer>, point: Point) {
    ui.point_at(point);
    let _ = ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
}

fn drag_to<'a, M: 'a>(ui: &mut Simulator<'a, M, Theme, Renderer>, point: Point) {
    ui.point_at(point);
    let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: point })]);
}

fn release<'a, M: 'a>(ui: &mut Simulator<'a, M, Theme, Renderer>) {
    let _ = ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
}

#[test]
fn pressing_a_thumb_centre_keeps_its_value() {
    for orientation in [Horizontal, Vertical] {
        for value in [0, 1, 7, 33, 50, 67, 99, 100] {
            let mut ui = drawn(slider::slider(0..=100).default_value(value), orientation);
            let point = at(orientation, center(value));
            press(&mut ui, point);
            drag_to(&mut ui, point);
            release(&mut ui);
            assert_eq!(
                ui.into_messages().collect::<Vec<_>>(),
                Vec::<u8>::new(),
                "{orientation:?} at {value}"
            );
        }
    }
}

#[test]
fn tapping_a_thumb_centre_keeps_its_value() {
    for orientation in [Horizontal, Vertical] {
        for value in [0, 33, 100] {
            let mut ui = drawn(slider::slider(0..=100).default_value(value), orientation);
            let position = at(orientation, center(value));
            let id = touch::Finger(1);
            ui.point_at(position);
            let _ = ui.simulate([
                Event::Touch(touch::Event::FingerPressed {
                    id,
                    position,
                    timestamp: None,
                }),
                Event::Touch(touch::Event::FingerMoved {
                    id,
                    position,
                    timestamp: None,
                }),
                Event::Touch(touch::Event::FingerLifted {
                    id,
                    position,
                    timestamp: None,
                }),
            ]);
            assert_eq!(
                ui.into_messages().collect::<Vec<_>>(),
                Vec::<u8>::new(),
                "{orientation:?}"
            );
        }
    }
}

#[test]
fn pressing_the_centre_of_a_range_thumb_keeps_both_values() {
    for orientation in [Horizontal, Vertical] {
        let mut ui = drawn(slider::range(0..=100).default_value([23, 71]), orientation);
        for value in [23, 71] {
            press(&mut ui, at(orientation, center(value)));
            release(&mut ui);
        }
        assert_eq!(
            ui.into_messages().collect::<Vec<_>>(),
            Vec::<[u8; 2]>::new(),
            "{orientation:?}"
        );
    }
}

#[test]
fn clicking_the_track_ends_gives_the_bounds() {
    for orientation in [Horizontal, Vertical] {
        let mut ui = drawn(slider::slider(0..=100).default_value(50), orientation);
        for offset in [0.0, 3.0, LENGTH - 3.0, LENGTH - 0.5, 0.5] {
            press(&mut ui, at(orientation, offset));
            release(&mut ui);
        }
        assert_eq!(
            ui.into_messages().collect::<Vec<_>>(),
            [0, 100, 0],
            "{orientation:?}"
        );
    }
}

#[test]
fn clicking_the_track_centres_the_thumb_on_the_pointer() {
    for orientation in [Horizontal, Vertical] {
        let mut ui = drawn(slider::slider(0..=100).default_value(90), orientation);
        press(&mut ui, at(orientation, center(25)));
        release(&mut ui);
        assert_eq!(
            ui.into_messages().collect::<Vec<_>>(),
            [25],
            "{orientation:?}"
        );
    }
}

#[test]
fn clicking_the_track_snaps_to_the_step() {
    let mut ui = drawn(
        slider::slider(0..=100).default_value(0).step(25),
        Horizontal,
    );
    press(&mut ui, at(Horizontal, center(60)));
    release(&mut ui);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [50]);
}

#[test]
fn dragging_a_thumb_keeps_the_grab_offset() {
    for orientation in [Horizontal, Vertical] {
        let mut ui = drawn(slider::slider(0..=100).default_value(50), orientation);
        let grab = center(50) + 5.0;
        press(&mut ui, at(orientation, grab));
        drag_to(&mut ui, at(orientation, grab + 10.4));
        drag_to(&mut ui, at(orientation, LENGTH));
        drag_to(&mut ui, at(orientation, -20.0));
        release(&mut ui);
        assert_eq!(
            ui.into_messages().collect::<Vec<_>>(),
            [60, 100, 0],
            "{orientation:?}"
        );
    }
}

#[test]
fn stacked_thumbs_part_in_the_direction_of_the_drag() {
    for (to, expected) in [(30, [30, 50]), (70, [50, 70])] {
        let mut ui = drawn(slider::range(0..=100).default_value([50, 50]), Horizontal);
        press(&mut ui, at(Horizontal, center(50)));
        drag_to(&mut ui, at(Horizontal, center(to)));
        release(&mut ui);
        assert_eq!(ui.into_messages().collect::<Vec<_>>(), [expected]);
    }
}

#[test]
fn a_disabled_slider_ignores_presses() {
    let mut ui = drawn(
        slider::slider(0..=100).default_value(50).disabled(true),
        Horizontal,
    );
    press(&mut ui, at(Horizontal, center(50)));
    drag_to(&mut ui, at(Horizontal, center(80)));
    release(&mut ui);
    press(&mut ui, at(Horizontal, 0.0));
    release(&mut ui);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), Vec::<u8>::new());
}
