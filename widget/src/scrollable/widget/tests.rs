use super::{Direction, Interaction, Scrollbars, Source, State, Swipe};
use iced_core::time::{Duration, Instant};
use iced_core::{Event, Point, Rectangle, Size, Vector, mouse, touch};

const VIEW: Rectangle = Rectangle {
    x: 0.,
    y: 0.,
    width: 160.,
    height: 100.,
};
const CONTENT: Size = Size {
    width: 160.,
    height: 600.,
};
const FINGER: touch::Finger = touch::Finger(1);

fn event(state: &mut State, touch: touch::Event, visible: bool) {
    let cursor = mouse::Cursor::Available(Point::new(30., 50.));
    let over = visible.then_some(Point::new(30., 50.));
    let direction = Direction::default();
    let event = Event::Touch(touch);
    let rails = Scrollbars::new(Vector::ZERO, direction, VIEW, CONTENT);
    state.interact(
        &event, VIEW, CONTENT, cursor, over, &rails, None, direction, true, false,
    );
    state.update(
        &event, VIEW, CONTENT, cursor, over, None, direction, true, false, false,
    );
}

fn start() -> State {
    let mut state = State::new();
    event(
        &mut state,
        touch::Event::FingerPressed {
            id: FINGER,
            position: Point::new(30., 90.),
        },
        true,
    );
    state
}

fn drag(state: &mut State) {
    event(
        state,
        touch::Event::FingerMoved {
            id: FINGER,
            position: Point::new(30., 30.),
        },
        true,
    );
}

#[test]
fn swipe_coasts_with_touch_notifications_and_stops_at_the_content_edge() {
    let mut state = start();
    drag(&mut state);
    assert_eq!(state.axis_offset(super::Axis::Y, VIEW, CONTENT), 60.);
    event(
        &mut state,
        touch::Event::FingerLifted {
            id: FINGER,
            position: Point::new(30., 30.),
        },
        true,
    );
    let started = state.segment.started;
    assert_eq!(state.source, Some(Source::Touch));
    assert!(state.step(started + Duration::from_millis(16), VIEW, CONTENT));
    assert!(state.axis_offset(super::Axis::Y, VIEW, CONTENT) > 60.);
    assert_eq!(
        state.notify_viewport(VIEW, CONTENT).unwrap().source,
        Source::Touch
    );
    let smaller = Size::new(160., 250.);
    state.step(started + Duration::from_millis(32), VIEW, smaller);
    state.step(started + Duration::from_secs(2), VIEW, smaller);
    assert_eq!(state.axis_offset(super::Axis::Y, VIEW, smaller), 150.);
    assert!(state.target.is_none());
}

#[test]
fn holds_taps_and_cancellations_stop_without_inertia() {
    for (moving, cancel) in [(false, false), (true, true), (true, false)] {
        let mut state = start();
        if moving {
            drag(&mut state);
        }
        if moving
            && !cancel
            && let Interaction::TouchScrolling(swipe) = &mut state.interaction
        {
            swipe.moved_at -= Duration::from_millis(150);
        }
        let position = Point::new(30., 30.);
        event(
            &mut state,
            if cancel {
                touch::Event::FingerLost {
                    id: FINGER,
                    position,
                }
            } else {
                touch::Event::FingerLifted {
                    id: FINGER,
                    position,
                }
            },
            true,
        );
        assert!(state.target.is_none());
    }
}

#[test]
fn only_the_dragging_finger_can_launch_a_coast_and_a_visible_new_press_stops_it() {
    let mut state = start();
    drag(&mut state);
    let position = Point::new(30., 30.);
    let other = touch::Finger(99);
    event(
        &mut state,
        touch::Event::FingerMoved {
            id: other,
            position: Point::ORIGIN,
        },
        true,
    );
    event(
        &mut state,
        touch::Event::FingerLifted {
            id: other,
            position,
        },
        true,
    );
    assert!(matches!(state.interaction, Interaction::TouchScrolling(_)));
    assert_eq!(state.axis_offset(super::Axis::Y, VIEW, CONTENT), 60.);
    event(
        &mut state,
        touch::Event::FingerLifted {
            id: FINGER,
            position,
        },
        true,
    );
    assert!(state.target.is_some());
    event(
        &mut state,
        touch::Event::FingerPressed {
            id: other,
            position,
        },
        false,
    );
    assert!(state.target.is_some());
    event(
        &mut state,
        touch::Event::FingerPressed {
            id: other,
            position,
        },
        true,
    );
    assert!(state.target.is_none());
}

#[test]
fn coasting_at_the_boundary_or_along_a_non_scrolling_axis_schedules_no_frames() {
    let mut state = State::new();
    let now = Instant::now();
    for velocity in [Vector::new(3000., 0.), Vector::new(0., -500.)] {
        let swipe = Swipe {
            finger: FINGER,
            position: Point::ORIGIN,
            moved_at: now,
            velocity,
        };
        state.fling(swipe, Direction::default(), VIEW, CONTENT, now);
        assert!(state.target.is_none());
    }
}
