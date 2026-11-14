//! Regression coverage for joined surfaces and the installed Button Group recipes.
use browser::Message;
use browser::component::button_group::ButtonGroupPage;
use browser::page::Page;
use iced::keyboard::key::Named;
use iced::{Rectangle, Size};
use iced_test::core::Settings;
use iced_test::core::widget::Id;
use iced_test::core::widget::operation::{Operation, Outcome, focusable};
use iced_test::{Simulator, simulator};
use icedcn::Theme;

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
fn click_control(ui: &mut Preview<'_>, index: usize) {
    let bounds = controls(ui)[index];
    ui.point_at(bounds.center());
    ui.simulate(simulator::click());
    ui.draw(&Theme::light());
}
fn close<'a>(mut ui: Preview<'a>, page: &'a dyn Page, id: &str, label: &str) -> Preview<'a> {
    ui.tap_key(Named::Escape);
    // Overlay Component events invalidate widgets; the native application rebuilds
    // the view. The Simulator requires that application step explicitly.
    let mut ui = ui.rebuild(page.preview(id));
    ui.draw(&Theme::light());
    assert!(ui.find(label).is_err(), "{label} remains after Escape");
    ui
}

#[test]
fn geometry_keeps_inputs_visible_at_both_widths_and_uses_the_window_breakpoint() {
    let mut page = ButtonGroupPage::new(0);
    for (window, width) in [(1440., 558.), (390., 260.)] {
        page.window_width(window);
        for (id, count) in [
            ("demo", if window >= 640. { 5 } else { 4 }),
            ("orientation", 2),
            ("size", 12),
            ("nested", 2),
            ("separator", 2),
            ("split", 2),
            ("input", 2),
            ("input-group", 3),
            ("dropdown", 2),
            ("select", 3),
            ("popover", 2),
        ] {
            let mut ui = preview(&*page, id, width);
            let bounds = controls(&mut ui);
            assert_eq!(bounds.len(), count, "{id} at {window}: {bounds:?}");
            assert!(
                bounds.iter().all(|b| b.width > 0.
                    && b.height > 0.
                    && b.x >= 0.
                    && b.x + b.width <= width + 0.1),
                "{id} at {window}: {bounds:?}"
            );
            if id == "size" {
                assert_eq!(bounds[0].x, bounds[4].x);
                assert_eq!(bounds[4].x, bounds[8].x);
            }
            if id == "orientation" {
                assert_eq!(bounds[0].width, 32.);
                assert_eq!(bounds[1].y, 32.);
            }
            if id == "input" {
                assert_eq!(bounds[0].width, if window < 768. { 216. } else { 192. });
                assert!((bounds[1].x - bounds[0].width).abs() < 0.01);
            }
            if id == "nested" || id == "input-group" {
                assert!(
                    bounds[1].width > 160.,
                    "visible message control: {bounds:?}"
                );
            }
            if id == "select" {
                assert!((bounds[1].x - 50.406).abs() < 0.1);
                assert!(bounds[1].width > 160.);
            }
        }
    }
}

#[test]
fn voice_preserves_text_while_disabling_only_the_input() {
    let page = ButtonGroupPage::new(0);
    let mut ui = preview(&*page, "input-group", 260.);
    click_control(&mut ui, 1);
    ui.typewrite("Hello");
    ui.find("Hello").expect("typed message");
    click_control(&mut ui, 2);
    assert_eq!(
        controls(&mut ui).len(),
        2,
        "voice leaves Plus and the toggle enabled"
    );
    ui.typewrite("ignored");
    click_control(&mut ui, 1);
    assert_eq!(controls(&mut ui).len(), 3);
    ui.find("Hello")
        .expect("uncontrolled text survives voice on/off");
    assert!(ui.find("Helloignored").is_err());
}

#[test]
fn joined_search_and_currency_remain_editable_and_select_retains_its_value() {
    let page = ButtonGroupPage::new(0);
    let mut search = preview(&*page, "input", 260.);
    click_control(&mut search, 0);
    search.typewrite("query");
    search.find("query").expect("editable search");
    let mut ui = preview(&*page, "select", 281.5);
    click_control(&mut ui, 0);
    ui.click("Euro").expect("choose Euro");
    let mut ui = ui.rebuild(page.preview("select"));
    ui.draw(&Theme::light());
    ui.find("€").expect("selected symbol");
    click_control(&mut ui, 1);
    ui.typewrite("12.50");
    ui.find("12.50").expect("editable amount");
    click_control(&mut ui, 0);
    ui.find("Euro").expect("options reopen");
    ui = close(ui, &*page, "select", "Euro");
    ui.find("€").expect("selection survives dismissal");
}

#[test]
fn overlay_triggers_accept_keyboard_and_dismiss_without_losing_the_group() {
    let page = ButtonGroupPage::new(0);
    for (id, label) in [
        ("dropdown", "Mute Conversation"),
        ("popover", "Start a new task with Copilot"),
    ] {
        let mut ui = preview(&*page, id, 340.);
        next(&mut ui);
        next(&mut ui);
        ui.tap_key(Named::Enter);
        ui.draw(&Theme::light());
        ui.find(label).expect("keyboard opens joined trigger");
        ui = close(ui, &*page, id, label);
        assert_eq!(controls(&mut ui).len(), 2);
        ui.tap_key(Named::Space);
        ui.find(label).expect("focus returns to the joined trigger");
        close(ui, &*page, id, label);
    }
}

#[test]
fn hero_submenu_keeps_the_label_after_reopening() {
    let page = ButtonGroupPage::new(0);
    let mut ui = preview(&*page, "demo", 558.);
    click_control(&mut ui, 4);
    ui.click("Label As...").expect("label submenu");
    ui.click("Work").expect("choose Work");
    let mut ui = ui.rebuild(page.preview("demo"));
    click_control(&mut ui, 4);
    ui.click("Label As...").expect("reopen label submenu");
    let work = ui
        .find("Work")
        .expect("label choice remains present")
        .bounds();
    let check = ui
        .find("✓")
        .expect("selected label has a checkmark")
        .bounds();
    assert_eq!(work.center_y(), check.center_y());
    ui.tap_key(Named::Escape);
    ui.tap_key(Named::Escape);
    ui.find("Archive").expect("group remains mounted");
}
