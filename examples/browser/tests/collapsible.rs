//! Focused Collapsible frame, lifecycle and nested interaction regressions.
use browser::Message;
use browser::component::collapsible::CollapsiblePage;
use browser::page::Page;
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::{Event, Point, Rectangle, Size, window};
use iced_test::core::Settings;
use iced_test::core::widget::Id;
use iced_test::core::widget::operation::{Operation, Outcome, focusable};
use iced_test::{Simulator, selector, simulator};
use icedcn::Theme;

type Ui<'a> = Simulator<'a, Message, Theme, iced::Renderer>;
fn preview<'a>(page: &'a dyn Page, id: &str, width: f32) -> Ui<'a> {
    Simulator::with_size(
        Settings::default(),
        Size::new(width, 1800.),
        page.docs_preview(id, &Theme::light()),
    )
}
fn frame(ui: &mut Ui<'_>) {
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
}
fn click(ui: &mut Ui<'_>, point: Point) {
    ui.point_at(point);
    ui.simulate(simulator::click());
    frame(ui);
}
fn next(ui: &mut Ui<'_>) -> Rectangle {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
    ui.find(selector::is_focused())
        .expect("focused control")
        .bounds()
}
fn near(a: f32, b: f32) {
    assert!((a - b).abs() < 0.1, "{a} != {b}");
}

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
fn controls(ui: &mut Ui<'_>) -> Vec<Rectangle> {
    let mut op = Controls::default();
    ui.operate(&mut op);
    op.0
}

#[test]
fn source_frames_shrink_at_second_width_and_hero_repeats_without_a_closed_gap() {
    let page = CollapsiblePage::new(0);
    for width in [638., 340.] {
        let mut ui = preview(&*page, "demo", width);
        frame(&mut ui);
        let trigger = controls(&mut ui)[0];
        near(trigger.y, 40.);
        near(trigger.width, 32.);
        let shipped = ui.find("Shipped").unwrap().bounds();
        near(shipped.y, 89.);
        assert!(trigger.x + trigger.width <= width - 40.);
        for _ in 0..2 {
            click(&mut ui, trigger.center());
            ui.find("100 Market St, San Francisco").unwrap();
            ui.find("2x Studio Headphones").unwrap();
            near(ui.find("Shipped").unwrap().bounds().y, shipped.y);
            click(&mut ui, trigger.center());
            assert!(ui.find("Shipping address").is_err());
        }
        for (id, min_height) in [("basic", 288.), ("settings", 288.), ("file-tree", 576.)] {
            let mut other = preview(&*page, id, width);
            frame(&mut other);
            for bounds in controls(&mut other) {
                assert!(
                    bounds.x >= 39. && bounds.x + bounds.width <= width - 39.,
                    "{id}: {bounds:?} at {width}"
                );
                assert!(bounds.y + bounds.height < min_height, "{id}: {bounds:?}");
            }
        }
    }
}

#[test]
fn basic_keyboard_reveals_body_and_survives_resize_and_theme_rebuild() {
    let page = CollapsiblePage::new(0);
    let mut ui = preview(&*page, "basic", 638.);
    frame(&mut ui);
    let trigger = next(&mut ui);
    near(trigger.y, 56.);
    ui.tap_key(Named::Space);
    frame(&mut ui);
    ui.find("Learn More").unwrap();
    ui = ui.resize(Size::new(340., 1800.));
    ui = ui.rebuild(page.docs_preview("basic", &Theme::dark()));
    frame(&mut ui);
    ui.find("Learn More").unwrap();
    let text = ui
        .find("This panel can be expanded or collapsed to reveal additional content.")
        .unwrap()
        .bounds();
    assert!(text.x + text.width <= 300.);
    ui.tap_key(Named::Enter);
    frame(&mut ui);
    assert!(ui.find("Learn More").is_err());
}

#[test]
fn settings_mounts_four_inputs_and_resets_only_the_hidden_row() {
    let page = CollapsiblePage::new(0);
    let mut ui = preview(&*page, "settings", 638.);
    frame(&mut ui);
    let original = controls(&mut ui);
    assert_eq!(original.len(), 3);
    near(original[0].width, 124.);
    near(original[0].y, original[2].y);
    click(&mut ui, original[0].center());
    ui.typewrite("1");
    click(&mut ui, original[2].center());
    let open = controls(&mut ui);
    assert_eq!(open.len(), 5);
    near(open[2].y - open[0].y, 40.);
    near(open[2].width, 124.);
    click(&mut ui, open[2].center());
    ui.typewrite("2");
    click(&mut ui, open[4].center());
    assert_eq!(controls(&mut ui).len(), 3);
    click(&mut ui, original[2].center());
    let reopened = controls(&mut ui);
    assert_eq!(reopened.len(), 5);
    // The upper input remains mounted; the bottom two get new default values.
    ui.find("01").expect("upper row retains edit");
    assert!(ui.find("02").is_err(), "lower input is unmounted on close");
}

#[test]
fn nested_folders_are_independent_and_remount_closed_with_full_fixture() {
    let page = CollapsiblePage::new(0);
    let mut ui = preview(&*page, "file-tree", 638.);
    frame(&mut ui);
    for label in [
        "components",
        "lib",
        "state",
        "types",
        "public",
        "main.rs",
        "layout.rs",
        "theme.rs",
        "Cargo.toml",
        "rustfmt.toml",
        "README.md",
        ".gitignore",
    ] {
        ui.find(label).unwrap();
    }
    ui.click("components").unwrap();
    frame(&mut ui);
    ui.click("ui").unwrap();
    frame(&mut ui);
    for file in [
        "button.rs",
        "card.rs",
        "dialog.rs",
        "input.rs",
        "select.rs",
        "table.rs",
        "login_form.rs",
        "register_form.rs",
    ] {
        ui.find(file).unwrap();
    }
    let parent = ui.find("components").unwrap().bounds();
    let child = ui.find("ui").unwrap().bounds();
    near(child.x - parent.x, 20.);
    ui.click("lib").unwrap();
    frame(&mut ui);
    for file in ["util.rs", "style.rs", "api.rs", "button.rs"] {
        ui.find(file).unwrap();
    }
    ui.click("components").unwrap();
    frame(&mut ui);
    assert!(ui.find("button.rs").is_err());
    ui.click("components").unwrap();
    frame(&mut ui);
    ui.find("ui").unwrap();
    assert!(ui.find("button.rs").is_err());
    ui.find("util.rs").unwrap();
    ui.click("Outline").unwrap();
    frame(&mut ui);
    ui.find("util.rs").unwrap();
    ui.click("Explorer").unwrap();
    frame(&mut ui);
    ui.find(".gitignore").unwrap();
}
