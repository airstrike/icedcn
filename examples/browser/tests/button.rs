//! Focused geometry and consumer regressions for the installed Button page.
use browser::Message;
use browser::component::button::ButtonPage;
use browser::page::Page;
use iced::keyboard::key::Named;
use iced::{Point, Rectangle, Size};
use iced_test::core::Settings;
use iced_test::core::widget::Id;
use iced_test::core::widget::operation::{Operation, Outcome, focusable};
use iced_test::{Simulator, simulator};
use icedxdy::Theme;

type Preview<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

#[derive(Default)]
struct Controls(Vec<Rectangle>);
impl Operation for Controls {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn focusable::Focusable) {
        self.0.push(bounds);
    }
}
fn controls(ui: &mut Preview<'_>) -> Vec<Rectangle> {
    let mut op = Controls::default();
    ui.operate(&mut op);
    op.0
}
fn next(ui: &mut Preview<'_>) {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
}
fn preview<'a>(page: &'a dyn Page, id: &str, width: f32) -> Preview<'a> {
    Simulator::with_size(
        Settings {
            text_size: 14.into(),
            ..Settings::default()
        },
        Size::new(width, 288.),
        page.preview(id),
    )
}

#[test]
fn size_uses_window_breakpoint_and_retains_all_eight_controls() {
    let mut page = ButtonPage::new(0);
    for (window, width, horizontal) in [
        (1440., 558., true),
        (640., 558., true),
        (639., 324., false),
        (390., 260., false),
    ] {
        page.window_width(window);
        let mut ui = preview(&*page, "size", width);
        let bounds = controls(&mut ui);
        assert_eq!(bounds.len(), 8);
        for (index, pair) in bounds.chunks_exact(2).enumerate() {
            let h = 24. + 4. * index as f32;
            assert_eq!(pair[0].height, h);
            assert_eq!(pair[1].size(), Size::new(h, h));
            assert!((pair[1].x - pair[0].x - pair[0].width - 8.).abs() < 0.01);
            assert_eq!(pair[0].y, pair[1].y);
            assert!(pair[1].x + pair[1].width <= width);
            if index > 0 {
                assert_eq!(pair[0].y == bounds[0].y, horizontal);
                if !horizontal {
                    assert_eq!(pair[0].x, bounds[0].x);
                }
            }
        }
    }
}

#[test]
fn spinner_spacing_fits_narrow_frame_and_loading_controls_are_not_focusable() {
    let page = ButtonPage::new(0);
    for width in [558., 260.] {
        let mut ui = preview(&*page, "spinner", width);
        ui.draw(&Theme::light());
        let first = ui.find("Generating").expect("first loading label").bounds();
        let second = ui
            .find("Downloading")
            .expect("second loading label")
            .bounds();
        assert_eq!(first.y, second.y);
        // Remaining padding 11 + outer gap 8 + next leading padding 9.
        assert!((second.x - first.x - first.width - 28.).abs() < 0.01);
        assert!(second.x + second.width + 16. + 6. + 11. <= width);
        assert!(controls(&mut ui).is_empty());
        next(&mut ui);
        ui.tap_key(Named::Enter);
        ui.tap_key(Named::Space);
        assert_eq!(ui.drain().count(), 0);
    }
}

#[test]
fn anchor_and_handlerless_examples_preserve_the_public_action_contract() {
    let page = ButtonPage::new(0);
    let mut ui = preview(&*page, "link-button", 260.);
    ui.point_at(Point::new(12., 12.));
    ui.simulate(simulator::click());
    assert!(matches!(
        ui.drain().next(),
        Some(Message::OpenUrl("https://github.com/login"))
    ));
    ui.point_at(Point::new(220., 200.));
    ui.simulate(simulator::click());
    next(&mut ui);
    ui.tap_key(Named::Enter);
    assert!(matches!(
        ui.drain().next(),
        Some(Message::OpenUrl("https://github.com/login"))
    ));
    for id in [
        "demo",
        "default",
        "outline",
        "secondary",
        "ghost",
        "destructive",
        "link",
        "icon",
        "with-icon",
        "rounded",
    ] {
        let mut ui = preview(&*page, id, 260.);
        assert!(!controls(&mut ui).is_empty());
        ui.point_at(Point::new(12., 12.));
        for _ in 0..2 {
            ui.simulate(simulator::click());
        }
        assert_eq!(ui.drain().count(), 0);
        assert!(!controls(&mut ui).is_empty());
    }
}
