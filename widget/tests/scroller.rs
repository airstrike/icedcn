//! Rebuilding a streaming transcript must preserve the reader's place.
use iced_core::time::{Duration, Instant};
use iced_core::widget::{Id, Operation, operation};
use iced_core::{Element, Event, Point, Rectangle, Size, Vector, mouse, touch, window};
use iced_runtime::task;
use iced_test::simulator::{Simulator, simulator};
use iced_widget::{Renderer, Space};
use icedcn_core::Theme;
use icedcn_widget::scroller::{Animation, Position, Reader, Target};
use icedcn_widget::{container, scroller};

#[derive(Default)]
struct Offset(f32);
impl Operation for Offset {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        _: Rectangle,
        _: Size,
        translation: Vector,
        _: &mut dyn operation::Scrollable,
    ) {
        self.0 = translation.y;
    }
}
fn jump(ui: &mut Simulator<'_, Reader, Theme, Renderer>, target: Target) {
    let task = scroller::scroll_to("chat", target, Animation::Instant);
    use iced_runtime::futures::futures::StreamExt;
    use iced_runtime::futures::futures::executor::block_on;
    let mut actions = task::into_stream(task).unwrap();
    let Some(iced_runtime::Action::<()>::Widget(mut op)) = block_on(actions.next()) else {
        panic!("expected scroll operation")
    };
    ui.operate(&mut op);
}
fn transcript(
    ids: &[u64],
    last_height: u32,
    start: Position,
    anchors: &[u64],
) -> Element<'static, Reader, Theme, Renderer> {
    let length = ids.len();
    let rows = ids.iter().enumerate().map(|(i, &id)| {
        scroller::item(
            id,
            Space::new()
                .height(if i + 1 == length { last_height } else { 60 })
                .width(200),
        )
        .anchor(anchors.contains(&id))
    });
    container(
        scroller(rows)
            .padding(0.0)
            .spacing(0.0)
            .height(100)
            .scroll_margin(0.0)
            .default_scroll_position(start)
            .auto_scroll(true)
            .id("chat")
            .on_reader_change(|r| r),
    )
    .width(200)
    .into()
}
#[test]
fn follows_output_until_a_message_jump_and_preserves_prepended_history() {
    let items = [1, 2, 3, 4];
    let mut ui = simulator(transcript(&items, 60, Position::End, &[]));
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 140.0);
    let mut ui = ui.rebuild(transcript(&items, 120, Position::End, &[]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 200.0);
    jump(&mut ui, Target::Item(2));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 60.0);
    let mut ui = ui.rebuild(transcript(&[0, 1, 2, 3, 4], 160, Position::End, &[]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 120.0);
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    let reader = ui.into_messages().last().unwrap();
    assert_eq!(reader.visible, [2, 3]);
    assert!(reader.can_scroll_up && reader.can_scroll_down);
}
#[test]
fn waits_for_missing_command_target_and_makes_room_for_a_new_turn() {
    let mut ui = simulator(transcript(&[], 60, Position::Start, &[]));
    jump(&mut ui, Target::Item(5));
    let mut ui = ui.rebuild(transcript(&[1, 2, 3, 4, 5], 60, Position::Start, &[1, 5]));
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 240.0);
    let mut ui = ui.rebuild(transcript(
        &[1, 2, 3, 4, 5, 6],
        80,
        Position::Start,
        &[1, 5],
    ));
    ui.operate(&mut offset);
    // An explicit item jump releases following, including when the queued target appears.
    assert_eq!(offset.0, 240.0);
    let ui = simulator(transcript(&[1, 2], 60, Position::End, &[1]));
    let ui = ui.rebuild(transcript(&[1, 2, 3], 60, Position::End, &[1, 3]));
    let mut ui = ui.rebuild(transcript(&[1, 2, 3, 4], 120, Position::End, &[1, 3]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 200.0);
}

#[test]
fn reading_with_keyboard_releases_and_reengages_follow_at_the_live_edge() {
    use iced_core::Point;
    use iced_core::keyboard::key::Named;
    use iced_test::simulator;
    let items = [1, 2, 3, 4];
    let mut ui = simulator(transcript(&items, 60, Position::End, &[]));
    ui.point_at(Point::new(30., 30.));
    let _ = ui.simulate(simulator::click());
    ui.tap_key(Named::PageUp);
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 40.0);
    let mut ui = ui.rebuild(transcript(&items, 120, Position::End, &[]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 40.0);
    ui.tap_key(Named::End);
    let mut ui = ui.rebuild(transcript(&items, 140, Position::End, &[]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 220.0);
}

#[test]
fn missing_jump_after_mount_does_not_hijack_later_appends() {
    let mut ui = simulator(transcript(&[1, 2, 3], 60, Position::Start, &[]));
    jump(&mut ui, Target::Item(99));
    let mut ui = ui.rebuild(transcript(&[1, 2, 3, 99], 60, Position::Start, &[]));
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 0.0);
}

#[test]
fn public_defaults_open_at_the_last_turn_and_do_not_follow_token_growth() {
    let render = |reply_height: u32| -> Element<'static, (), Theme, Renderer> {
        let rows = [(1, 60), (2, 60), (3, 60), (4, reply_height)]
            .into_iter()
            .map(|(id, height)| {
                scroller::item(id, Space::new().width(100).height(height))
                    .anchor(id == 1 || id == 3)
            });
        scroller(rows).spacing(0.).padding(0.).height(100).into()
    };
    let mut ui = simulator(render(60));
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 104.);
    let mut ui = ui.rebuild(render(200));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 104.);
}

#[test]
fn repeated_jumps_to_the_same_key_work_after_reader_navigation() {
    use iced_core::Point;
    use iced_core::keyboard::key::Named;
    use iced_test::simulator::click;
    let mut ui = simulator(transcript(&[1, 2, 3], 60, Position::Start, &[]));
    ui.point_at(Point::new(30., 30.));
    let _ = ui.simulate(click());
    let mut offset = Offset::default();
    for _ in 0..2 {
        jump(&mut ui, Target::Item(2));
        ui.operate(&mut offset);
        assert_eq!(offset.0, 60.);
        ui.tap_key(Named::Home);
        ui.operate(&mut offset);
        assert_eq!(offset.0, 0.);
    }
}

#[test]
fn scrollable_keeps_child_taps_and_coasts_after_an_unhandled_swipe() {
    use iced_core::{Point, time::Duration, touch};
    use icedcn_widget::{button, column, scrollable};

    let body = scrollable(column![
        button("Tap").width(100).height(28).on_press(true),
        Space::new().height(600).width(160),
    ])
    .height(100)
    .width(160);
    let mut ui = simulator(Element::<bool, Theme, Renderer>::from(body));
    let id = touch::Finger(1);
    let button = Point::new(12., 12.);
    ui.point_at(button);
    let _ = ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            timestamp: None,
            id,
            position: button,
        }),
        Event::Touch(touch::Event::FingerLifted {
            timestamp: None,
            id,
            position: button,
        }),
    ]);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);

    // Without platform timestamps the velocity tracker samples the wall
    // clock, and a batch dispatched within ~1µs reads as no motion at all.
    let start = Point::new(24., 90.);
    let end = Point::new(24., 48.);
    ui.point_at(start);
    let _ = ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            timestamp: Some(Duration::ZERO),
            id,
            position: start,
        }),
        Event::Touch(touch::Event::FingerMoved {
            id,
            position: end,
            timestamp: Some(Duration::from_millis(16)),
        }),
        Event::Touch(touch::Event::FingerLifted {
            id,
            position: end,
            timestamp: Some(Duration::from_millis(32)),
        }),
    ]);
    let now = Instant::now();
    let mut offset = Offset::default();
    let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(now))]);
    ui.operate(&mut offset);
    let released = offset.0;
    assert!(released >= 42.);
    let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(
        now + Duration::from_millis(100),
    ))]);
    ui.operate(&mut offset);
    assert!(offset.0 > released + 12.);
    assert_eq!(ui.drain().count(), 0);
}

/// A wide snippet at the top of a tall article, with a real tappable child.
fn nested_scrolls(content_width: u32) -> Simulator<'static, bool, Theme, Renderer> {
    use icedcn_widget::{button, column, scrollable};
    let snippet = scrollable(column![
        button("Tap").width(140).height(36).on_press(true),
        Space::new().width(content_width).height(36),
    ])
    .id("snippet")
    .horizontal()
    .height(72)
    .width(160);
    let article = scrollable(column![snippet, Space::new().height(800).width(160)])
        .id("article")
        .height(150)
        .width(160);
    simulator(Element::from(article))
}

#[derive(Default)]
struct NestedOffsets {
    article: f32,
    snippet: f32,
    snippet_y: f32,
}
impl Operation for NestedOffsets {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn scrollable(
        &mut self,
        id: Option<&Id>,
        _: Rectangle,
        _: Size,
        translation: Vector,
        _: &mut dyn operation::Scrollable,
    ) {
        if id == Some(&Id::new("snippet")) {
            self.snippet = translation.x;
            self.snippet_y = translation.y;
        } else if id == Some(&Id::new("article")) {
            self.article = translation.y;
        }
    }
}
fn offsets(ui: &mut Simulator<'_, bool, Theme, Renderer>) -> (f32, f32) {
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    let mut offsets = NestedOffsets::default();
    ui.operate(&mut offsets);
    (offsets.snippet, offsets.article)
}
type Nested = Simulator<'static, bool, Theme, Renderer>;
const FINGER: touch::Finger = touch::Finger(1);

fn contact(ui: &mut Nested, event: touch::Event, position: Point) {
    ui.point_at(position);
    let _ = ui.simulate([Event::Touch(event)]);
}

fn press(ui: &mut Nested, x: f32, y: f32) {
    let position = Point::new(x, y);
    contact(
        ui,
        touch::Event::FingerPressed {
            timestamp: None,
            id: FINGER,
            position,
        },
        position,
    );
}

fn drag(ui: &mut Nested, x: f32, y: f32) {
    let position = Point::new(x, y);
    contact(
        ui,
        touch::Event::FingerMoved {
            timestamp: None,
            id: FINGER,
            position,
        },
        position,
    );
}

fn lift(ui: &mut Nested, x: f32, y: f32) {
    let position = Point::new(x, y);
    contact(
        ui,
        touch::Event::FingerLifted {
            timestamp: None,
            id: FINGER,
            position,
        },
        position,
    );
}

#[test]
fn diagonal_swipes_choose_one_nested_viewport_and_cancel_its_button() {
    for (x, y, expected) in [(83., 6., (0., 24.)), (58., 22., (32., 0.))] {
        let mut ui = nested_scrolls(440);
        press(&mut ui, 90., 30.);
        drag(&mut ui, x, y);
        assert_eq!(offsets(&mut ui), expected);
        lift(&mut ui, x, y);
        assert_eq!(ui.drain().count(), 0, "release over original button");
    }
    let mut ui = nested_scrolls(440);
    press(&mut ui, 80., 30.);
    drag(&mut ui, 79., 29.);
    lift(&mut ui, 79., 29.);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
    assert_eq!(offsets(&mut ui), (0., 0.));

    let (start, end) = (Point::new(80., 30.), Point::new(52., 28.));
    for (event, position) in [
        (
            touch::Event::FingerPressed {
                id: FINGER,
                position: start,
                timestamp: Some(Duration::ZERO),
            },
            start,
        ),
        (
            touch::Event::FingerMoved {
                id: FINGER,
                position: end,
                timestamp: Some(Duration::from_millis(16)),
            },
            end,
        ),
        (
            touch::Event::FingerLifted {
                id: FINGER,
                position: end,
                timestamp: Some(Duration::from_millis(32)),
            },
            end,
        ),
    ] {
        contact(&mut ui, event, position);
    }
    let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(
        Instant::now() + Duration::from_millis(80),
    ))]);
    let mut coast = NestedOffsets::default();
    ui.operate(&mut coast);
    assert!(coast.snippet > 28. && coast.article == 0.);
}

#[test]
fn reaching_a_child_edge_does_not_replay_a_drag_on_the_parent() {
    let mut ui = nested_scrolls(170);
    press(&mut ui, 90., 60.);
    drag(&mut ui, 62., 51.);
    assert_eq!(offsets(&mut ui), (10., 0.));
    drag(&mut ui, 52., 15.);
    assert_eq!(offsets(&mut ui), (10., 0.));
    lift(&mut ui, 52., 15.);
    for width in [150, 440] {
        let mut ui = nested_scrolls(width);
        press(&mut ui, 90., 60.);
        drag(&mut ui, 90., 21.);
        assert_eq!(offsets(&mut ui), (0., 39.));
    }
}

#[test]
fn horizontal_thumb_owns_its_drag_over_the_vertical_article() {
    let mut ui = nested_scrolls(440);
    press(&mut ui, 20., 71.);
    drag(&mut ui, 62., 32.);
    let (snippet, article) = offsets(&mut ui);
    assert!(snippet > 60., "thumb drag moved the snippet: {snippet}");
    assert_eq!(article, 0.);
    lift(&mut ui, 62., 32.);
    assert_eq!(offsets(&mut ui), (snippet, 0.));
}

#[test]
fn wheel_rechecks_axis_and_bounds_on_every_event_over_a_snippet() {
    let mut ui = nested_scrolls(440);
    ui.point_at(Point::new(80., 15.));
    for (x, y, expected) in [
        (-16., -3., (16., 0.)),
        (-1., -12., (16., 12.)),
        (-8., 0., (24., 12.)),
        (500., 0., (0., 12.)),
        (10., 2., (0., 12.)),
        (0., 6., (0., 6.)),
    ] {
        let _ = ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Pixels { x, y },
        })]);
        assert_eq!(offsets(&mut ui), expected);
    }
}

#[test]
fn a_new_vertical_gesture_chains_when_the_inner_viewport_is_at_its_edge() {
    use icedcn_widget::{column, scrollable};
    let pane = scrollable(Space::new().width(160).height(82))
        .height(72)
        .width(160);
    let article = scrollable(column![pane, Space::new().width(160).height(500)])
        .id("article")
        .width(160)
        .height(150);
    let mut ui = simulator(Element::from(article));
    press(&mut ui, 50., 60.);
    drag(&mut ui, 50., 28.);
    drag(&mut ui, 50., 14.);
    lift(&mut ui, 50., 14.);
    assert_eq!(
        offsets(&mut ui).1,
        0.,
        "the first gesture stays with its owner"
    );
    press(&mut ui, 50., 60.);
    drag(&mut ui, 50., 38.);
    assert_eq!(offsets(&mut ui).1, 22., "the next gesture starts cleanly");
}

/// The first flick starts in the article's blank area; the next lands on the
/// nested pane's button. This exercises real child-first widget dispatch.
fn continuation_tree(horizontal: bool) -> Nested {
    use icedcn_widget::{button, column, scrollable};
    let pane = scrollable(column![
        button("Tap").width(140).height(100).on_press(true),
        Space::new()
            .width(if horizontal { 500 } else { 160 })
            .height(1000),
    ])
    .id("snippet")
    .height(200)
    .width(160);
    let pane = if horizontal { pane.horizontal() } else { pane };
    let article = scrollable(column![
        Space::new().height(200).width(160),
        pane,
        Space::new().height(1000).width(160),
    ])
    .id("article")
    .height(400)
    .width(160);
    simulator(Element::from(article))
}

fn nested_snapshot(ui: &mut Nested) -> NestedOffsets {
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    let mut snapshot = NestedOffsets::default();
    ui.operate(&mut snapshot);
    snapshot
}

fn fling_article(ui: &mut Nested) -> Point {
    for (ms, y) in [(0, 100.), (10, 90.), (20, 80.), (30, 70.), (40, 70.)] {
        let position = Point::new(100., y);
        let timestamp = Some(Duration::from_millis(ms));
        let event = match ms {
            0 => touch::Event::FingerPressed {
                id: FINGER,
                position,
                timestamp,
            },
            40 => touch::Event::FingerLifted {
                id: FINGER,
                position,
                timestamp,
            },
            _ => touch::Event::FingerMoved {
                id: FINGER,
                position,
                timestamp,
            },
        };
        contact(ui, event, position);
    }
    let _ = offsets(ui);
    let snapshot = nested_snapshot(ui);
    assert!(snapshot.article >= 30. && snapshot.article < 100.);
    Point::new(100., 240. - snapshot.article)
}

#[test]
fn a_second_flick_over_a_child_continues_the_active_parent_and_cancels_its_button() {
    let mut ui = continuation_tree(false);
    let point = fling_article(&mut ui);
    press(&mut ui, point.x, point.y);
    let before = nested_snapshot(&mut ui);
    drag(&mut ui, point.x, point.y - 1.);
    assert_eq!(nested_snapshot(&mut ui).article, before.article);
    drag(&mut ui, point.x, point.y - 40.);
    let after = nested_snapshot(&mut ui);
    assert!((after.article - before.article - 40.).abs() < 0.01);
    assert_eq!(after.snippet_y, 0.);
    lift(&mut ui, point.x, point.y - 40.);
    assert_eq!(
        ui.drain().count(),
        0,
        "the parent claim cancels the child's button"
    );
}

#[test]
fn a_tap_stops_the_parent_and_the_next_gesture_can_scroll_the_child() {
    let mut ui = continuation_tree(false);
    let point = fling_article(&mut ui);
    press(&mut ui, point.x, point.y);
    lift(&mut ui, point.x, point.y);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [true],
        "an unclaimed tap still reaches its button"
    );
    let before = nested_snapshot(&mut ui);
    press(&mut ui, point.x, point.y);
    drag(&mut ui, point.x, point.y - 40.);
    let after = nested_snapshot(&mut ui);
    assert_eq!(after.article, before.article);
    assert_eq!(after.snippet_y, 40.);
}

#[test]
fn a_cancelled_contact_does_not_leave_parent_fling_priority_behind() {
    let mut ui = continuation_tree(false);
    let point = fling_article(&mut ui);
    press(&mut ui, point.x, point.y);
    contact(
        &mut ui,
        touch::Event::FingerLost {
            id: FINGER,
            position: point,
            timestamp: None,
        },
        point,
    );
    let before = nested_snapshot(&mut ui);
    press(&mut ui, point.x, point.y);
    drag(&mut ui, point.x, point.y - 40.);
    let after = nested_snapshot(&mut ui);
    assert_eq!(after.article, before.article);
    assert_eq!(after.snippet_y, 40.);
}

#[test]
fn reversing_the_interrupted_fling_allows_the_child_to_scroll() {
    let mut ui = continuation_tree(false);
    press(&mut ui, 100., 260.);
    drag(&mut ui, 100., 220.);
    contact(
        &mut ui,
        touch::Event::FingerLost {
            id: FINGER,
            position: Point::new(100., 220.),
            timestamp: None,
        },
        Point::new(100., 220.),
    );
    let point = fling_article(&mut ui);
    press(&mut ui, point.x, point.y);
    let before = nested_snapshot(&mut ui);
    assert_eq!(before.snippet_y, 40.);
    drag(&mut ui, point.x, point.y + 20.);
    let after = nested_snapshot(&mut ui);
    assert_eq!(after.article, before.article);
    assert_eq!(after.snippet_y, 20.);
}

#[test]
fn an_orthogonal_drag_over_a_child_does_not_continue_the_parent_fling() {
    let mut ui = continuation_tree(true);
    let point = fling_article(&mut ui);
    press(&mut ui, point.x, point.y);
    let before = nested_snapshot(&mut ui);
    drag(&mut ui, point.x - 40., point.y);
    let after = nested_snapshot(&mut ui);
    assert_eq!(after.article, before.article);
    assert_eq!(after.snippet, 40.);
}

#[test]
fn a_finished_parent_fling_does_not_reserve_the_next_contact() {
    let mut ui = continuation_tree(false);
    fling_article(&mut ui);
    let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(
        Instant::now() + Duration::from_secs(10),
    ))]);
    let before = nested_snapshot(&mut ui);
    let point = Point::new(100., 240. - before.article);
    assert!(point.y > 0. && point.y < 400.);
    press(&mut ui, point.x, point.y);
    drag(&mut ui, point.x, point.y - 10.);
    let after = nested_snapshot(&mut ui);
    assert_eq!(after.article, before.article);
    assert_eq!(after.snippet_y, 10.);
}
