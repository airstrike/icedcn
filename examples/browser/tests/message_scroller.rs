//! Navigate the same transcript fixtures that the component browser installs.
use std::borrow::Cow;
use std::time::Duration;

use browser::component::message_scroller::MessageScrollerPage;
use browser::page::Page;
use browser::{Message, icon};
use iced::time::Instant;
use iced::{Event, Rectangle, Size, Vector, window};
use iced_test::core::Settings;
use iced_test::simulator::Simulator;
use iced_widget::core::widget::{Id, Operation, operation};
use icedxdy::Theme;

type Preview<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn preview<'a>(page: &'a dyn Page, id: &str) -> Preview<'a> {
    let mut settings = Settings::default();
    settings.fonts.push(Cow::Borrowed(icon::FONT));
    Simulator::with_size(settings, Size::new(425., 800.), page.preview(id))
}

#[derive(Default, Debug)]
struct Geometry {
    y: f32,
    content: f32,
    view: f32,
}
impl Operation for Geometry {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        translation: Vector,
        _: &mut dyn operation::Scrollable,
    ) {
        if bounds.height >= self.view {
            self.y = translation.y;
            self.content = content.height;
            self.view = bounds.height;
        }
    }
}
fn geometry(ui: &mut Preview<'_>) -> Geometry {
    let mut result = Geometry::default();
    ui.operate(&mut result);
    assert!(result.view > 0., "expected transcript viewport");
    result
}
fn close(a: f32, b: f32) {
    assert!((a - b).abs() < 1., "{a} differs from {b}");
}

#[test]
fn saved_thread_can_open_at_the_start_end_or_last_user_turn() {
    let page = MessageScrollerPage::new(0);
    let mut ui = preview(&*page, "opening-position");
    let initial = geometry(&mut ui);
    assert!(initial.y > 0.);
    ui.click("start").unwrap();
    close(geometry(&mut ui).y, 0.);
    ui.click("end").unwrap();
    let end = geometry(&mut ui);
    close(end.y, end.content - end.view);
    ui.click("last-anchor").unwrap();
    close(geometry(&mut ui).y, initial.y);
}

#[test]
fn load_history_keeps_visible_turn_in_place() {
    let page = MessageScrollerPage::new(0);
    let mut ui = preview(&*page, "load-history");
    let before = geometry(&mut ui);
    ui.click(Id::from("transcript-load-history")).unwrap();
    ui.find("History Loaded").unwrap();
    let after = geometry(&mut ui);
    assert!(after.content > before.content);
    close(after.y - before.y, after.content - before.content);
}

#[test]
fn adding_turns_anchors_and_streaming_replies_follow_the_live_edge() {
    let page = MessageScrollerPage::new(0);
    let send = icon::ALL_ICONS
        .iter()
        .find(|(name, _)| *name == "arrow_up")
        .unwrap()
        .1;
    let mut ui = preview(&*page, "anchoring");
    for _ in 0..3 {
        ui.click(send).unwrap();
    }
    let anchored = geometry(&mut ui);
    assert!(anchored.y > 0.);
    close(anchored.y, anchored.content - anchored.view);

    let mut ui = preview(&*page, "streaming");
    let start = Instant::now();
    let mut frame = 0;
    for _ in 0..2 {
        ui.click(send).unwrap();
        for _ in 0..150 {
            frame += 1;
            let now = start + Duration::from_millis(frame * 32);
            let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(now))]);
        }
    }
    let streamed = geometry(&mut ui);
    assert!(streamed.y > 0., "stream produced scrollable content");
    close(streamed.y, streamed.content - streamed.view);
}

#[test]
fn jump_menu_targets_a_named_turn_and_keyboard_status_tracks_scroll() {
    use iced::Point;
    use iced::keyboard::key::Named;
    use iced_test::simulator;
    let page = MessageScrollerPage::new(0);
    let mut ui = preview(&*page, "commands");
    let bottom = geometry(&mut ui).y;
    assert!(bottom > 0.);
    ui.click("Jump to...").unwrap();
    let text: String = "We're seeing activation dip after workspace creation."
        .chars()
        .take(39)
        .collect();
    ui.click(format!("{text}...")).unwrap();
    // The enclosing application processes the overlay's invalidation, then
    // the native scroll animation advances on redraws.
    let mut ui = ui.rebuild(page.preview("commands"));
    let start = Instant::now();
    for step in 0..30 {
        let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(
            start + Duration::from_millis(step * 32),
        ))]);
    }
    assert!(geometry(&mut ui).y < 10.);

    let mut ui = preview(&*page, "scrollable");
    ui.point_at(Point::new(60., 166.));
    let _ = ui.simulate(simulator::click());
    ui.find("You are at the top. You can only scroll down.")
        .unwrap();
    ui.tap_key(Named::PageDown);
    ui.find("You can scroll both ways.").unwrap();
    ui.tap_key(Named::End);
    ui.find("You are at the bottom. You can only scroll up.")
        .unwrap();
}
