use super::{
    Axis, Direction, Interaction, Offset, Scrollbar, Scrollbars, Source, State, Swipe, Target,
};
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

fn timestamp(milliseconds: u64) -> Option<Duration> {
    Some(Duration::from_millis(10_000 + milliseconds))
}

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
            timestamp: timestamp(0),
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
            timestamp: timestamp(30),
        },
        true,
    );
}

fn lift(state: &mut State, milliseconds: u64) {
    event(
        state,
        touch::Event::FingerLifted {
            id: FINGER,
            position: Point::new(30., 30.),
            timestamp: timestamp(milliseconds),
        },
        true,
    );
}

fn trace(velocity: Vector, now: Instant, dispatch_ms: [u64; 3]) -> Swipe {
    let mut swipe = Swipe::new(FINGER, Point::ORIGIN, timestamp(0), now);
    for (i, dispatched) in dispatch_ms.into_iter().enumerate() {
        let milliseconds = (i as u64 + 1) * 10;
        let position = Point::ORIGIN - velocity * (milliseconds as f32 / 1000.);
        swipe.drag(
            position,
            timestamp(milliseconds),
            now + Duration::from_millis(dispatched),
        );
    }
    swipe
}

fn launch(velocity: Vector, content: Size, now: Instant) -> State {
    let mut state = State::new();
    state.fling(
        trace(velocity, now, [10, 20, 30]),
        Direction::default(),
        VIEW,
        content,
        now + Duration::from_millis(31),
        timestamp(31),
    );
    state
}

fn near(actual: f32, expected: f64) {
    assert!(
        (f64::from(actual) - expected).abs() < 0.02,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn swipe_coasts_with_touch_notifications_and_stops_when_content_shrinks() {
    let mut state = start();
    drag(&mut state);
    assert_eq!(state.axis_offset(Axis::Y, VIEW, CONTENT), 60.);
    lift(&mut state, 31);
    let started = state.segment.started;
    assert_eq!(state.source, Some(Source::Touch));
    assert!(state.step(started + Duration::from_millis(16), VIEW, CONTENT));
    assert!(state.axis_offset(Axis::Y, VIEW, CONTENT) > 60.);
    assert_eq!(
        state.notify_viewport(VIEW, CONTENT).unwrap().source,
        Source::Touch
    );
    let smaller = Size::new(160., 250.);
    state.step(started + Duration::from_millis(32), VIEW, smaller);
    assert!(!state.step(started + Duration::from_secs(2), VIEW, smaller));
    assert_eq!(state.axis_offset(Axis::Y, VIEW, smaller), 150.);
    assert!(state.target.is_none());
    assert!(state.fling.is_none());
}

#[test]
fn release_velocity_drives_the_chromium_trajectory_and_exact_final_offset() {
    let content = Size::new(160., 10_000.);
    let mut state = launch(Vector::new(0., 2000.), content, Instant::now());
    let started = state.segment.started;
    // Independently generated from pinned Chromium MobileScroller C++;
    // see fling.rs for the oracle revision and generation procedure.
    near(state.target.unwrap().destination.y, 647.398_804);
    near(state.segment.duration, 0.924_855);
    assert!(state.step(started + Duration::from_millis(500), VIEW, content));
    near(state.axis_offset(Axis::Y, VIEW, content), 572.792_48);
    assert!(!state.step(started + Duration::from_secs(2), VIEW, content));
    near(state.axis_offset(Axis::Y, VIEW, content), 647.398_804);
    assert!(state.target.is_none());
    assert!(state.fling.is_none());
    assert!(state.last_frame.is_none());
}

#[test]
fn original_timestamps_make_batched_and_delayed_dispatch_equivalent() {
    let now = Instant::now();
    let content = Size::new(160., 10_000.);
    let velocity = Vector::new(0., 2000.);
    let mut states = [State::new(), State::new(), State::new()];
    for (state, (dispatch, release)) in
        states
            .iter_mut()
            .zip([([10, 20, 30], 31), ([0, 0, 0], 0), ([70, 90, 200], 350)])
    {
        state.fling(
            trace(velocity, now, dispatch),
            Direction::default(),
            VIEW,
            content,
            now + Duration::from_millis(release),
            timestamp(31),
        );
        near(state.target.unwrap().destination.y, 647.398_804);
        let started = state.segment.started;
        state.step(started + Duration::from_millis(500), VIEW, content);
        near(state.axis_offset(Axis::Y, VIEW, content), 572.792_48);
    }
    assert_eq!(states[0].offset_y, states[1].offset_y);
    assert_eq!(states[1].offset_y, states[2].offset_y);
}

#[test]
fn sixty_and_one_hundred_twenty_hertz_frames_share_the_same_trajectory() {
    let content = Size::new(160., 10_000.);
    let now = Instant::now();
    let mut sixty = launch(Vector::new(0., 2000.), content, now);
    let mut one_twenty = launch(Vector::new(0., 2000.), content, now);
    let started = sixty.segment.started;
    for frame in 1..=120 {
        let time = started + Duration::from_nanos(frame * 1_000_000_000 / 120);
        let active = one_twenty.step(time, VIEW, content);
        if frame % 2 == 0 {
            assert_eq!(sixty.step(time, VIEW, content), active);
            assert_eq!(sixty.offset_y, one_twenty.offset_y);
        }
    }
    assert!(sixty.target.is_none());
    assert!(one_twenty.target.is_none());
    near(sixty.axis_offset(Axis::Y, VIEW, content), 647.398_804);
}

#[test]
fn an_edge_clips_the_original_motion_without_compressing_launch_velocity() {
    let now = Instant::now();
    let content = Size::new(160., 250.);
    let mut state = launch(Vector::new(0., 2000.), content, now);
    let started = state.segment.started;
    assert_eq!(state.target.unwrap().destination.y, 150.);
    assert!(state.step(started + Duration::from_micros(16_666), VIEW, content));
    near(state.axis_offset(Axis::Y, VIEW, content), 33.283_477_8);
    assert!(!state.step(started + Duration::from_millis(100), VIEW, content));
    assert_eq!(state.axis_offset(Axis::Y, VIEW, content), 150.);
    assert!(state.last_frame.is_none());
}

#[test]
fn a_reverse_fling_stops_at_zero_without_a_negative_target_or_offset() {
    let now = Instant::now();
    let mut state = State::new();
    state.offset_y = Offset::Absolute(150.);
    state.fling(
        trace(Vector::new(0., -2000.), now, [10, 20, 30]),
        Direction::default(),
        VIEW,
        CONTENT,
        now + Duration::from_millis(31),
        timestamp(31),
    );
    let started = state.segment.started;
    assert_eq!(state.target.unwrap().destination.y, 0.);
    assert!(state.step(started + Duration::from_micros(16_666), VIEW, CONTENT));
    near(state.axis_offset(Axis::Y, VIEW, CONTENT), 116.716_522_2);
    assert!(!state.step(started + Duration::from_millis(100), VIEW, CONTENT));
    assert_eq!(state.offset_y, Offset::Absolute(0.));
    assert!(state.target.is_none());
    assert!(state.fling.is_none());
    assert!(state.last_frame.is_none());
}

#[test]
fn a_wheel_retarget_leaves_the_fling_and_settles_on_its_new_target() {
    let content = Size::new(160., 10_000.);
    let mut state = launch(Vector::new(0., 2000.), content, Instant::now());
    let now = state.segment.started + Duration::from_millis(500);
    state.step(now, VIEW, content);
    state.scroll_smoothly(Vector::new(0., 60.), VIEW, content, now);
    assert!(state.fling.is_none());
    near(state.target.unwrap().destination.y, 707.398_804);
    assert!(!state.step(now + Duration::from_secs(2), VIEW, content));
    near(state.axis_offset(Axis::Y, VIEW, content), 707.398_804);
}

#[test]
fn a_clipped_horizontal_velocity_cannot_reverse_a_vertical_retarget() {
    let now = Instant::now();
    let content = Size::new(10_000., 10_000.);
    let mut state = State::new();
    state.fling(
        trace(Vector::new(-2400., 1800.), now, [10, 20, 30]),
        Direction::Both {
            horizontal: Scrollbar::default(),
            vertical: Scrollbar::default(),
        },
        VIEW,
        content,
        now + Duration::from_millis(31),
        timestamp(31),
    );
    let now = state.segment.started + Duration::from_millis(100);
    assert!(state.step(now, VIEW, content));
    assert_eq!(state.axis_offset(Axis::X, VIEW, content), 0.);
    assert!(state.animated_velocity(now) > 0.);
    let before = state.axis_offset(Axis::Y, VIEW, content);
    state.retarget(
        Target::new(
            Offset::Absolute(0.),
            Offset::Absolute(before + 200.),
            VIEW,
            content,
        ),
        now,
    );
    assert!(state.fling.is_none());
    assert!(state.segment.slope > 0.);
    assert!(state.step(now + Duration::from_millis(1), VIEW, content));
    assert!(state.axis_offset(Axis::Y, VIEW, content) > before);
    assert_eq!(state.axis_offset(Axis::X, VIEW, content), 0.);
}

#[test]
fn holds_taps_and_cancellations_stop_without_inertia() {
    for (moving, cancel) in [(false, false), (true, true), (true, false)] {
        let mut state = start();
        if moving {
            drag(&mut state);
        }
        let position = Point::new(30., 30.);
        event(
            &mut state,
            if cancel {
                touch::Event::FingerLost {
                    id: FINGER,
                    position,
                    timestamp: timestamp(31),
                }
            } else {
                touch::Event::FingerLifted {
                    id: FINGER,
                    position,
                    timestamp: timestamp(110),
                }
            },
            true,
        );
        assert!(state.target.is_none());
        assert!(state.fling.is_none());
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
            timestamp: timestamp(31),
        },
        true,
    );
    event(
        &mut state,
        touch::Event::FingerLifted {
            id: other,
            position,
            timestamp: timestamp(32),
        },
        true,
    );
    assert!(matches!(state.interaction, Interaction::TouchScrolling(_)));
    assert_eq!(state.axis_offset(Axis::Y, VIEW, CONTENT), 60.);
    lift(&mut state, 33);
    assert!(state.target.is_some());
    for visible in [false, true] {
        event(
            &mut state,
            touch::Event::FingerPressed {
                id: other,
                position,
                timestamp: timestamp(34),
            },
            visible,
        );
        assert_eq!(state.target.is_none(), visible);
        assert_eq!(state.fling.is_none(), visible);
    }
}

#[test]
fn boundary_non_scrolling_axes_and_subthreshold_releases_schedule_no_frames() {
    for velocity in [
        Vector::new(3000., 0.),
        Vector::new(0., -500.),
        Vector::new(0., 40.),
    ] {
        let state = launch(velocity, CONTENT, Instant::now());
        assert!(state.target.is_none());
        assert!(state.fling.is_none());
        assert!(state.last_frame.is_none());
    }
}

#[test]
fn an_interrupted_parent_at_its_edge_does_not_preempt_the_child() {
    let mut state = launch(Vector::new(0., 2000.), CONTENT, Instant::now());
    event(
        &mut state,
        touch::Event::FingerPressed {
            id: FINGER,
            position: Point::new(30., 80.),
            timestamp: timestamp(100),
        },
        true,
    );
    // The available range may change between the press and the first move.
    state.offset_y = Offset::Absolute(500.);
    let move_event = Event::Touch(touch::Event::FingerMoved {
        id: FINGER,
        position: Point::new(30., 40.),
        timestamp: timestamp(110),
    });
    let direction = Direction::default();
    let rails = Scrollbars::new(Vector::ZERO, direction, VIEW, CONTENT);
    let cursor = mouse::Cursor::Available(Point::new(30., 40.));
    let interaction = state.interact(
        &move_event,
        VIEW,
        CONTENT,
        cursor,
        Some(Point::new(30., 40.)),
        &rails,
        None,
        direction,
        true,
        false,
    );
    assert!(
        interaction.content.is_some(),
        "a saturated owner must defer to children"
    );
    let update = state.update(
        &move_event,
        VIEW,
        CONTENT,
        cursor,
        Some(Point::new(30., 40.)),
        None,
        direction,
        true,
        false,
        false,
    );
    assert!(!update.capture);
    assert!(update.cancel_child.is_none());
    assert!(matches!(state.interaction, Interaction::None));
}

#[test]
fn an_elapsed_fling_does_not_reserve_a_contact_before_its_final_redraw() {
    let mut state = launch(
        Vector::new(0., 1000.),
        CONTENT,
        Instant::now() - Duration::from_secs(10),
    );
    event(
        &mut state,
        touch::Event::FingerPressed {
            id: FINGER,
            position: Point::new(30., 80.),
            timestamp: timestamp(100),
        },
        true,
    );
    let Interaction::TouchPending(swipe) = state.interaction else {
        panic!("the new contact should remain pending");
    };
    assert!(swipe.continuation.is_none());
}
