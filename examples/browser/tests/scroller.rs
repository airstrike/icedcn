//! Navigate the same transcript fixtures that the component browser installs.
use std::borrow::Cow;
use std::time::Duration;

use browser::component::scroller;
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
    preview_at(page, id, 425.)
}

fn preview_at<'a>(page: &'a dyn Page, id: &str, width: f32) -> Preview<'a> {
    let mut settings = Settings::default();
    settings.fonts.push(Cow::Borrowed(icon::FONT));
    Simulator::with_size(settings, Size::new(width, 800.), page.preview(id))
}

#[derive(Default, Debug)]
struct Geometry {
    y: f32,
    content: f32,
    view: f32,
    viewport: Rectangle,
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
            self.viewport = bounds;
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
    let page = scroller::Page::new(0);
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
    let page = scroller::Page::new(0);
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
    let page = scroller::Page::new(0);
    let send = glyph("arrow_up");
    let mut ui = preview(&*page, "anchoring");
    for _ in 0..3 {
        ui.click(send).unwrap();
    }
    let anchored = geometry(&mut ui);
    assert!(anchored.y > 0.);
    close(anchored.y, anchored.content - anchored.view);

    let mut ui = preview(&*page, "streaming");
    let mut now = Instant::now();
    for _ in 0..2 {
        ui.click(send).unwrap();
        advance(&mut ui, &mut now, 150);
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
    let page = scroller::Page::new(0);
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

/// Buttons report their interactive bounds, independent of the width of the text or icon.
#[derive(Default)]
struct Layout {
    buttons: Vec<Rectangle>,
    containers: Vec<Rectangle>,
}
impl Operation for Layout {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn container(&mut self, _: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
        self.containers.push(bounds);
    }

    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn operation::Focusable) {
        self.buttons.push(bounds);
    }
}
impl Layout {
    fn button_at(&self, label: Rectangle) -> Rectangle {
        self.buttons
            .iter()
            .copied()
            .find(|button| button.contains(label.center()))
            .expect("label is inside an enabled button")
    }
}
fn bounds(ui: &mut Preview<'_>, label: &str) -> Rectangle {
    ui.find(label)
        .expect("control label exists")
        .visible_bounds()
        .expect("control is on screen")
}
fn glyph(name: &str) -> &'static str {
    icon::ALL_ICONS
        .iter()
        .find(|(n, _)| *n == name)
        .expect("icon is registered")
        .1
}

#[test]
fn preview_controls_use_the_available_card_width() {
    let page = scroller::Page::new(0);
    let ids = [
        "demo",
        "anchoring",
        "group-chat",
        "previous-context",
        "streaming",
        "opening-position",
        "load-history",
        "animation",
        "commands",
        "visibility",
        "scrollable",
    ];
    for width in [320.0_f32, 700.] {
        for id in ids {
            let mut ui = preview_at(&*page, id, width);
            let mut layout = Layout::default();
            ui.operate(&mut layout);
            let available = width.min(384.);
            // Commands/visibility have no footer. The visibility outline may reserve one button's width.
            let card = layout
                .containers
                .iter()
                .filter(|r| r.height >= 500. && r.height <= 600. && r.y < 1.)
                .max_by(|a, b| a.width.total_cmp(&b.width))
                .expect("transcript card");
            let minimum = if id == "visibility" {
                available - 40.
            } else {
                available
            };
            assert!(
                card.width + 1. >= minimum && card.x + card.width <= width + 1.,
                "{id} in {width}: {card:?}"
            );
            if !matches!(id, "commands" | "visibility") {
                let full_width_footer = layout.containers.iter().any(|r| {
                    r.y > card.center_y()
                        && (r.y + r.height - card.y - card.height).abs() < 1.
                        && (r.x - card.x).abs() < 1.
                        && (r.width - card.width).abs() < 1.
                });
                assert!(
                    full_width_footer,
                    "{id} in {width}: footer does not span the card"
                );
            }
            for b in &layout.buttons {
                assert!(
                    b.x >= -1. && b.x + b.width <= width + 1.,
                    "{id} in {width}: horizontally clipped control {b:?}"
                );
            }
            match id {
                "demo" | "previous-context" | "streaming" | "anchoring" | "animation" => {
                    let send = layout.button_at(bounds(&mut ui, glyph("arrow_up")));
                    let inset = card.x + card.width - send.x - send.width;
                    assert!(
                        (0.0..=32.).contains(&inset),
                        "{id} in {width}: send inset {inset}"
                    );
                    if matches!(id, "demo" | "previous-context" | "streaming") {
                        let plus = layout.button_at(bounds(&mut ui, glyph("plus")));
                        close(send.center_y(), plus.center_y());
                        assert!(
                            (plus.x - card.x - inset).abs() <= 3.,
                            "{id} in {width}: asymmetric composer controls {plus:?} {send:?}"
                        );
                    }
                }
                "group-chat" | "load-history" => {
                    let b = if id == "group-chat" {
                        layout.button_at(bounds(&mut ui, "Add Rocky"))
                    } else {
                        let label = ui
                            .find(Id::from("transcript-load-history"))
                            .unwrap()
                            .bounds();
                        layout.button_at(label)
                    };
                    let inset = b.x - card.x;
                    assert!((0.0..=24.).contains(&inset), "{id}: leading inset {inset}");
                    close(inset, card.x + card.width - b.x - b.width);
                }
                "opening-position" => {
                    let first = layout.button_at(bounds(&mut ui, "start"));
                    let last = layout.button_at(bounds(&mut ui, "last-anchor"));
                    close((first.x + last.x + last.width) / 2., card.center_x());
                }
                _ => {}
            }
        }
    }
}
fn advance(ui: &mut Preview<'_>, clock: &mut Instant, frames: usize) {
    for _ in 0..frames {
        *clock += Duration::from_millis(32);
        ui.simulate([Event::Window(window::Event::RedrawRequested(*clock))]);
    }
}

#[test]
fn wheel_scrolling_up_keeps_the_current_position_while_the_reply_grows() {
    use iced::mouse::ScrollDelta;
    let page = scroller::Page::new(0);
    let mut ui = preview_at(&*page, "streaming", 320.);
    let send = glyph("arrow_up");
    let mut now = Instant::now();
    ui.click(send).unwrap();
    advance(&mut ui, &mut now, 150);
    ui.click(send).unwrap();
    advance(&mut ui, &mut now, 4);
    let live = geometry(&mut ui);
    ui.point_at(live.viewport.center());
    ui.scroll(ScrollDelta::Pixels { x: 0., y: 95. });
    advance(&mut ui, &mut now, 1);
    let reading = geometry(&mut ui);
    assert!(
        reading.y < live.y - 1.,
        "the wheel released the live edge ({live:?} -> {reading:?})"
    );
    advance(&mut ui, &mut now, 150);
    let done = geometry(&mut ui);
    assert!(
        done.content > reading.content,
        "further streamed lines lengthened the transcript"
    );
    close(reading.y, done.y);
}
