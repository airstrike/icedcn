//! Rebuilding a streaming transcript must preserve the reader's place.
use iced_core::time::Instant;
use iced_core::widget::{Id, Operation, operation};
use iced_core::{Element, Event, Rectangle, Size, Vector, window};
use iced_test::simulator::simulator;
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::message_scroller::{Command, Position, Reader, Target};
use icedxdy_widget::{container, message_scroller};

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
fn transcript(
    ids: &[u64],
    last_height: u32,
    start: Position,
    command: Option<Command>,
    anchors: &[u64],
) -> Element<'static, Reader, Theme, Renderer> {
    let length = ids.len();
    let rows = ids.iter().enumerate().map(|(i, &id)| {
        message_scroller::item(
            id,
            Space::new()
                .height(if i + 1 == length { last_height } else { 60 })
                .width(200),
        )
        .anchor(anchors.contains(&id))
    });
    container(
        message_scroller(rows)
            .padding(0.0)
            .spacing(0.0)
            .height(100)
            .scroll_margin(0.0)
            .default_scroll_position(start)
            .auto_scroll(true)
            .command(command)
            .on_reader_change(|r| r),
    )
    .width(200)
    .into()
}
#[test]
fn follows_output_until_a_message_jump_and_preserves_prepended_history() {
    let items = [1, 2, 3, 4];
    let mut ui = simulator(transcript(&items, 60, Position::End, None, &[]));
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 140.0);
    let mut ui = ui.rebuild(transcript(&items, 120, Position::End, None, &[]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 200.0);
    let command = Some(Command {
        sequence: 1,
        target: Target::Item(2),
    });
    let mut ui = ui.rebuild(transcript(&items, 120, Position::End, command, &[]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 60.0);
    let mut ui = ui.rebuild(transcript(
        &[0, 1, 2, 3, 4],
        160,
        Position::End,
        command,
        &[],
    ));
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
    let cmd = Some(Command {
        sequence: 2,
        target: Target::Item(5),
    });
    let ui = simulator(transcript(&[], 60, Position::Start, cmd, &[]));
    let mut ui = ui.rebuild(transcript(
        &[1, 2, 3, 4, 5],
        60,
        Position::Start,
        cmd,
        &[1, 5],
    ));
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 240.0);
    let mut ui = ui.rebuild(transcript(
        &[1, 2, 3, 4, 5, 6],
        80,
        Position::Start,
        cmd,
        &[1, 5],
    ));
    ui.operate(&mut offset);
    // An explicit item jump releases following, including when the queued target appears.
    assert_eq!(offset.0, 240.0);
    let ui = simulator(transcript(&[1, 2], 60, Position::End, None, &[1]));
    let ui = ui.rebuild(transcript(&[1, 2, 3], 60, Position::End, None, &[1, 3]));
    let mut ui = ui.rebuild(transcript(&[1, 2, 3, 4], 120, Position::End, None, &[1, 3]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 200.0);
}

#[test]
fn reading_with_keyboard_releases_and_reengages_follow_at_the_live_edge() {
    use iced_core::Point;
    use iced_core::keyboard::key::Named;
    use iced_test::simulator;
    let items = [1, 2, 3, 4];
    let mut ui = simulator(transcript(&items, 60, Position::End, None, &[]));
    ui.point_at(Point::new(30., 30.));
    let _ = ui.simulate(simulator::click());
    ui.tap_key(Named::PageUp);
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 40.0);
    let mut ui = ui.rebuild(transcript(&items, 120, Position::End, None, &[]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 40.0);
    ui.tap_key(Named::End);
    let mut ui = ui.rebuild(transcript(&items, 140, Position::End, None, &[]));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 220.0);
}

#[test]
fn missing_jump_after_mount_does_not_hijack_later_appends() {
    let cmd = Some(Command {
        sequence: 1,
        target: Target::Item(99),
    });
    let ui = simulator(transcript(&[1, 2, 3], 60, Position::Start, cmd, &[]));
    let mut ui = ui.rebuild(transcript(&[1, 2, 3, 99], 60, Position::Start, cmd, &[]));
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
                message_scroller::item(id, Space::new().width(100).height(height))
                    .anchor(id == 1 || id == 3)
            });
        message_scroller(rows)
            .spacing(0.)
            .padding(0.)
            .height(100)
            .into()
    };
    let mut ui = simulator(render(60));
    let mut offset = Offset::default();
    ui.operate(&mut offset);
    assert_eq!(offset.0, 104.);
    let mut ui = ui.rebuild(render(200));
    ui.operate(&mut offset);
    assert_eq!(offset.0, 104.);
}
