//! Geometry changes and reader gestures during keyed transcript updates.
use iced_core::keyboard::key::Named;
use iced_core::time::{Duration, Instant};
use iced_core::widget::{Id, Operation, operation};
use iced_core::{Element, Event, Point, Rectangle, Size, Vector, touch, window};
use iced_test::simulator::{Simulator, click, simulator};
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::scroller;
use icedxdy_widget::scroller::{Command, Position, Reader, Target};

type Preview = Simulator<'static, Reader, Theme, Renderer>;

fn view(
    rows: &[(u64, u32, bool)],
    position: Position,
    peek: f32,
    command: Option<Target>,
    smooth: bool,
) -> Element<'static, Reader, Theme, Renderer> {
    scroller(rows.iter().map(|&(id, height, anchor)| {
        scroller::item(id, Space::new().width(150).height(height)).anchor(anchor)
    }))
    .default_scroll_position(position)
    .height(100)
    .spacing(0.)
    .padding(0.)
    .scroll_margin(10.)
    .previous_item_peek(peek)
    .auto_scroll(true)
    .command(command.map(|target| Command {
        sequence: 1,
        target,
    }))
    .smooth_commands(smooth)
    .on_reader_change(|reader| reader)
    .into()
}

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
fn offset(ui: &mut Preview) -> f32 {
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    offset.0
}

#[test]
fn opening_a_short_last_turn_uses_the_real_end_even_with_previous_context() {
    let rows = [(1, 200, false), (2, 95, true)];
    let mut ui = simulator(view(&rows, Position::LastAnchor, 20., None, false));
    assert_eq!(offset(&mut ui), 195.);
}

#[test]
fn a_batch_of_new_turns_keeps_a_follower_at_the_end_and_otherwise_anchors_the_first_turn() {
    let initial = [(1, 140, false)];
    let batch = [
        (1, 140, false),
        (2, 90, true),
        (3, 90, false),
        (4, 80, true),
        (5, 110, false),
    ];
    for (position, expected) in [(Position::End, 410.), (Position::Start, 130.)] {
        let ui = simulator(view(&initial, position, 0., None, false));
        let mut ui = ui.rebuild(view(&batch, position, 0., None, false));
        assert_eq!(offset(&mut ui), expected);
    }
}

#[test]
fn prepending_preserves_the_visible_key_while_older_rows_rewrap_and_a_new_turn_arrives() {
    let initial = [(1, 100, false), (2, 100, true), (3, 100, true)];
    let cmd = Some(Target::Item(3));
    let ui = simulator(view(&initial, Position::Start, 0., cmd, false));
    let next = [
        (0, 30, false),
        (1, 180, false),
        (2, 100, true),
        (3, 100, true),
        (4, 50, true),
    ];
    let mut ui = ui.rebuild(view(&next, Position::Start, 0., cmd, false));
    assert_eq!(offset(&mut ui), 300.);
}

#[test]
fn an_explicit_item_jump_uses_margin_and_visibility_excludes_rows_above_the_reading_line() {
    let rows = [(1, 100, true), (2, 100, true), (3, 100, false)];
    let mut ui = simulator(view(
        &rows,
        Position::Start,
        20.,
        Some(Target::Item(2)),
        false,
    ));
    assert_eq!(offset(&mut ui), 90.);
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    let reading = ui.into_messages().last().unwrap();
    assert_eq!(reading.current_anchor, Some(2));
    assert_eq!(reading.visible, [2]);
}

#[test]
fn a_smooth_jump_keeps_its_key_when_history_arrives_before_it_settles() {
    let rows = [(1, 120, false), (2, 120, true), (3, 120, false)];
    let ui = simulator(view(&rows, Position::Start, 0., None, true));
    let mut ui = ui.rebuild(view(
        &rows,
        Position::Start,
        0.,
        Some(Target::Item(2)),
        true,
    ));
    assert!(offset(&mut ui) < 110.);
    let added = [
        (0, 80, false),
        (1, 120, false),
        (2, 120, true),
        (3, 120, false),
    ];
    let mut ui = ui.rebuild(view(
        &added,
        Position::Start,
        0.,
        Some(Target::Item(2)),
        true,
    ));
    let start = Instant::now();
    for n in 1..30 {
        let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(
            start + Duration::from_millis(n * 32),
        ))]);
    }
    assert_eq!(offset(&mut ui), 190.);
}

#[test]
fn touch_intent_at_the_live_edge_holds_position_before_the_next_chunk() {
    let before = [(1, 120, false), (2, 100, false)];
    let mut ui = simulator(view(&before, Position::End, 0., None, false));
    let position = Point::new(30., 30.);
    let _ = ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            id: touch::Finger(2),
            position,
        }),
        Event::Touch(touch::Event::FingerMoved {
            id: touch::Finger(2),
            position,
        }),
    ]);
    let mut ui = ui.rebuild(view(
        &[(1, 120, false), (2, 200, false)],
        Position::End,
        0.,
        None,
        false,
    ));
    assert_eq!(offset(&mut ui), 120.);
}

#[test]
fn a_placeholder_replaced_by_a_turn_of_the_same_length_is_anchored_once() {
    let ui = simulator(view(
        &[(1, 100, false), (2, 100, false)],
        Position::Start,
        20.,
        None,
        false,
    ));
    let rows = [(1, 100, false), (3, 100, true)];
    let mut ui = ui.rebuild(view(&rows, Position::Start, 20., None, false));
    assert_eq!(offset(&mut ui), 70.);
    ui.point_at(Point::new(30., 30.));
    let _ = ui.simulate(click());
    ui.tap_key(Named::Home);
    let mut ui = ui.rebuild(view(&rows, Position::Start, 20., None, false));
    assert_eq!(offset(&mut ui), 0.);
}

#[test]
fn keyboard_space_pages_the_focused_transcript() {
    let rows = [(1, 200, false), (2, 180, false)];
    let mut ui = simulator(view(&rows, Position::Start, 0., None, false));
    ui.point_at(Point::new(30., 30.));
    let _ = ui.simulate(click());
    ui.tap_key(Named::Space);
    assert_eq!(offset(&mut ui), 100.);
}

#[test]
fn keyed_editor_stays_with_its_message_across_a_reorder_and_replacement() {
    use icedxdy_widget::input;
    let editors = |ids: &[u64]| -> Element<'static, (), Theme, Renderer> {
        scroller(
            ids.iter()
                .map(|&id| scroller::item(id, input(format!("Reply {id}")).width(180))),
        )
        .padding(0.)
        .spacing(0.)
        .height(160)
        .into()
    };
    let mut ui = simulator(editors(&[1, 2]));
    ui.click("Reply 2").expect("select the second input");
    ui.typewrite("annotated");
    let mut ui = ui.rebuild(editors(&[2, 1]));
    let matched = ui.find("annotated").expect("draft survives move");
    assert!(
        matched.bounds().y < 30.,
        "draft belongs to message 2, now the first row"
    );
    let mut ui = ui.rebuild(editors(&[3, 1]));
    assert!(
        ui.find("annotated").is_err(),
        "replacement starts with a fresh input"
    );
}

#[test]
fn new_turn_uses_the_content_padding_and_peek_but_commands_use_only_padding_and_margin() {
    let initial = [(1, 113, true), (2, 237, false)];
    let with_prompt = [(1, 113, true), (2, 237, false), (3, 113, true)];
    let padded = |rows: &[(u64, u32, bool)],
                  target: Option<Target>|
     -> Element<'static, Reader, Theme, Renderer> {
        scroller(rows.iter().map(|&(key, height, anchor)| {
            scroller::item(key, Space::new().width(150).height(height)).anchor(anchor)
        }))
        .height(349)
        .spacing(32.)
        .padding(20.)
        .previous_item_peek(64.)
        .scroll_margin(0.)
        .default_scroll_position(Position::End)
        .command(target.map(|target| Command {
            sequence: 1,
            target,
        }))
        .on_reader_change(|reader| reader)
        .into()
    };
    // Same row heights as the live 384px card. The first prompt starts 20px
    // inside the viewport. The second prompt lands with 20 + 64 visible above.
    let mut ui = simulator(padded(&initial[..1], None));
    assert_eq!(offset(&mut ui), 0.);
    let ui = ui.rebuild(padded(&initial, None));
    let mut ui = ui.rebuild(padded(&with_prompt, None));
    assert_eq!(offset(&mut ui), 350.);
    let mut ui = ui.rebuild(padded(&with_prompt, Some(Target::Item(3))));
    assert_eq!(offset(&mut ui), 414.);
}

#[test]
fn remaining_tail_space_does_not_count_as_an_unread_message() {
    let rows = [(1, 120, false), (2, 40, true)];
    let mut ui = simulator(view(
        &rows,
        Position::Start,
        0.,
        Some(Target::Item(2)),
        false,
    ));
    ui.point_at(Point::new(30., 30.));
    let _ = ui.simulate(click());
    ui.tap_key(Named::ArrowUp);
    assert_eq!(offset(&mut ui), 70.);
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    let reader = ui.into_messages().last().unwrap();
    assert!(reader.can_scroll_up);
    assert!(!reader.can_scroll_down, "only reserved space remains below");
}

#[test]
fn a_tall_new_anchor_holds_while_a_short_turn_hands_back_to_following() {
    for (height, expected) in [(120, 110.), (50, 170.)] {
        let previous = [(1, 120, true)];
        let prompt = [(1, 120, true), (2, height, true)];
        let reply = [(1, 120, true), (2, height, true), (3, 100, false)];
        let ui = simulator(view(&previous, Position::End, 0., None, false));
        let mut ui = ui.rebuild(view(&prompt, Position::End, 0., None, false));
        assert_eq!(offset(&mut ui), 110.);
        let mut ui = ui.rebuild(view(&reply, Position::End, 0., None, false));
        assert_eq!(offset(&mut ui), expected);
    }
}

#[test]
fn clicking_to_focus_at_the_end_keeps_the_stream_following() {
    let mut ui = simulator(view(&[(1, 160, false)], Position::End, 0., None, false));
    ui.point_at(Point::new(30., 30.));
    let _ = ui.simulate(click());
    let mut ui = ui.rebuild(view(&[(1, 200, false)], Position::End, 0., None, false));
    assert_eq!(offset(&mut ui), 100.);
}
