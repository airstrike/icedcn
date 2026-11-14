//! Navigate the same transcript fixtures that the component browser installs.
use std::borrow::Cow;
use std::time::Duration;

use browser::component::scroller;
use browser::page::Page;
use browser::{Message, font, icon};
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
    settings
        .fonts
        .extend([icon::FONT, font::LUCIDE].map(Cow::Borrowed));
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
    close(initial.y, initial.content - initial.view);
    ui.click("start").unwrap();
    close(geometry(&mut ui).y, 0.);
    ui.click("end").unwrap();
    let end = geometry(&mut ui);
    close(end.y, end.content - end.view);
    ui.click("last-anchor").unwrap();
    let last = geometry(&mut ui);
    assert!(last.y > 0. && last.y < end.y);
    let label = bounds(
        &mut ui,
        "This is the last message the user sent in the conversation.",
    );
    // The text sits below its 1px bubble border and 10px vertical padding.
    close(label.y - last.viewport.y, 20. + 64. + 1. + 10.);
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
    for _ in 0..2 {
        ui.click(send).unwrap();
        let mut now = Instant::now();
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
            let available = if id == "visibility" {
                (width - 48.).min(384.)
            } else {
                width.min(384.)
            };
            let card = layout
                .containers
                .iter()
                .find(|r| (r.height - 560.).abs() < 1. && (r.width - available).abs() < 1.)
                .unwrap_or_else(|| panic!("{id} in {width}: card width should be {available}"));
            if !matches!(id, "commands" | "visibility") {
                let footer = layout.containers.iter().find(|r| {
                    r.y > card.center_y()
                        && (r.y + r.height - card.y - card.height + 20.).abs() < 1.
                        && (r.x - card.x).abs() < 1.
                        && (r.width - card.width).abs() < 1.
                });
                assert!(
                    footer.is_some(),
                    "{id} in {width}: footer should span the card above its 20px bottom inset"
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
                    let has_composer = matches!(id, "demo" | "previous-context" | "streaming");
                    close(inset, if has_composer { 31. } else { 20. });
                    close(send.width, 32.);
                    close(send.height, 32.);
                    if has_composer {
                        let group = ui.find(Id::from("transcript-composer")).unwrap().bounds();
                        close(group.x - card.x, 20.);
                        close(group.width, card.width - 40.);
                        close(group.height, 102.);
                        close(group.y + group.height - send.y - send.height, 9.);
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
                    close(inset, 20.);
                    close(b.height, 32.);
                    close(inset, card.x + card.width - b.x - b.width);
                }
                "opening-position" => {
                    let first = layout.button_at(bounds(&mut ui, "start"));
                    let last = layout.button_at(bounds(&mut ui, "last-anchor"));
                    close(first.x - card.x, 23.);
                    close(card.x + card.width - last.x - last.width, 23.);
                    close(first.width, last.width);
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
    now = Instant::now();
    advance(&mut ui, &mut now, 36);
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

#[test]
fn composer_menu_and_animation_picker_open_above_their_triggers() {
    let page = scroller::Page::new(0);
    let mut ui = preview(&*page, "demo");
    let plus = bounds(&mut ui, glyph("plus"));
    ui.click(glyph("plus")).unwrap();
    for label in [
        "Add Photos & Files",
        "Create Image",
        "Deep Research",
        "Web Search",
    ] {
        let item = bounds(&mut ui, label);
        assert!(item.y + item.height < plus.y);
    }
    ui.click("Deep Research").unwrap();
    let mut ui = ui.rebuild(page.preview("demo"));
    assert!(ui.find("Deep Research").is_err());

    let mut ui = preview(&*page, "animation");
    let trigger = bounds(&mut ui, "Fade");
    ui.click("Fade").unwrap();
    let pop = bounds(&mut ui, "Pop");
    assert!(pop.y < trigger.y);
    ui.click("Pop").unwrap();
    let mut ui = ui.rebuild(page.preview("animation"));
    close(bounds(&mut ui, "Pop").center_y(), trigger.center_y());
}

#[test]
fn send_paints_a_circle_inside_its_square_target() {
    use iced::{Color, Renderer, mouse};
    use iced_test::core::renderer::{Headless, Style};
    use iced_test::core::shell::{Bus, Waker};
    use iced_test::futures::futures::executor::block_on;
    use iced_test::runtime::UserInterface;

    let backend = std::env::var("ICED_TEST_BACKEND").ok();
    let mut renderer =
        block_on(Renderer::new(Default::default(), backend.as_deref())).expect("headless renderer");
    let page = scroller::Page::new(0);
    let theme = Theme::dark();
    let mut ui = UserInterface::build(
        page.themed_preview("animation", &theme),
        Size::new(384., 620.),
        Default::default(),
        &mut renderer,
    );
    ui.update(
        &window::Headless,
        &Waker::noop(),
        &[Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )],
        mouse::Cursor::Unavailable,
        &mut renderer,
        &mut Bus::new(),
    );
    let mut layout = Layout::default();
    ui.operate(&renderer, &mut layout);
    let send = layout.buttons.last().expect("the footer contains Send");
    assert_eq!(send.size(), Size::new(32., 32.));
    for scale in [1u32, 2] {
        ui.draw(
            &mut renderer,
            &theme,
            &Style::default(),
            mouse::Cursor::Unavailable,
        );
        let width = 384 * scale;
        let rgba = renderer.screenshot(Size::new(width, 620 * scale), scale as f32, Color::BLACK);
        let left = (send.x * scale as f32).round() as u32;
        let top = (send.y * scale as f32).round() as u32;
        let side = 32 * scale;
        let mut bounds = (side, side, 0, 0);
        for y in 0..side {
            for x in 0..side {
                let i = (4 * ((top + y) * width + left + x)) as usize;
                if rgba[i + 2].saturating_sub(rgba[i]) > 80 {
                    bounds = (
                        bounds.0.min(x),
                        bounds.1.min(y),
                        bounds.2.max(x),
                        bounds.3.max(y),
                    );
                }
            }
        }
        assert_eq!(
            bounds,
            (scale, scale, side - scale - 1, side - scale - 1),
            "the Rhea primary fill has a one-point transparent border on every side of {send:?} at {scale}×"
        );
    }
}

#[test]
fn ellipsis_uses_the_font_and_only_changes_the_last_overflowing_line() {
    use iced::mouse;
    use iced::{Color, Element, Font, Renderer};
    use iced_test::core::renderer::{Headless, Settings, Style};
    use iced_test::futures::futures::executor::block_on;
    use iced_test::runtime::UserInterface;
    use iced_widget::text::LineHeight;
    use icedxdy::widget::text::{self, Ellipsis};

    fn pixels(renderer: &mut Renderer, value: &str, lines: u32, ellipsis: Ellipsis) -> Vec<u8> {
        let height = lines * 20;
        let size = Size::new(72., height as f32);
        let label: Element<'_, (), Theme> = text::small(value)
            .width(72)
            .height(height as f32)
            .line_height(LineHeight::Absolute(20.into()))
            .ellipsis(ellipsis)
            .into();
        let mut ui = UserInterface::build(label, size, Default::default(), renderer);
        ui.draw(
            renderer,
            &Theme::light(),
            &Style::default(),
            mouse::Cursor::Unavailable,
        );
        renderer.screenshot(Size::new(72, height), 1., Color::WHITE)
    }

    let backend = std::env::var("ICED_TEST_BACKEND").ok();
    let render = |font| {
        block_on(Renderer::new(
            Settings {
                font,
                ..Default::default()
            },
            backend.as_deref(),
        ))
        .expect("headless renderer")
    };
    let mut proportional = render(Font::DEFAULT);
    let mut monospace = render(Font::MONOSPACE);
    let letters = "iii iii iii iii";
    assert_eq!(
        pixels(&mut proportional, letters, 1, Ellipsis::End),
        pixels(&mut proportional, letters, 1, Ellipsis::None),
        "the proportional letters fit inside one line"
    );
    assert!(
        pixels(&mut monospace, letters, 1, Ellipsis::End)
            != pixels(&mut monospace, letters, 1, Ellipsis::None),
        "the same characters in a wider font require an ellipsis"
    );

    let copy = "These two lines wrap at the available width, leaving later text undisplayed.";
    let shown = pixels(&mut proportional, copy, 2, Ellipsis::End);
    let plain = pixels(&mut proportional, copy, 2, Ellipsis::None);
    let row = 72 * 20 * 4;
    assert_eq!(shown[..row], plain[..row], "the first line is intact");
    assert!(
        shown[row..] != plain[row..],
        "the second line ends with an ellipsis"
    );
}
