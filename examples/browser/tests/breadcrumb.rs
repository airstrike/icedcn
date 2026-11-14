//! Focused regressions for the installed Breadcrumb recipes and their frames.
use browser::Message;
use browser::component::breadcrumb::BreadcrumbPage;
use browser::page::Page;
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::{Event, Point, Rectangle, Size, window};
use iced_test::core::Settings;
use iced_test::core::widget::Id;
use iced_test::core::widget::operation::{Operation, Outcome, focusable};
use iced_test::simulator;
use iced_test::{Simulator, selector};
use icedxdy::Theme;

type Preview<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn preview<'a>(page: &'a dyn Page, id: &str, width: f32) -> Preview<'a> {
    let settings = Settings {
        text_size: 14.into(),
        ..Settings::default()
    };
    Simulator::with_size(settings, Size::new(width, 300.), page.preview(id))
}

fn next(ui: &mut Preview<'_>) {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
}

fn frame(ui: &mut Preview<'_>) {
    ui.draw(&Theme::light());
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
}

#[derive(Default)]
struct Focused(Vec<Rectangle>);

impl Operation for Focused {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn focusable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        state: &mut dyn focusable::Focusable,
    ) {
        if state.is_focused() {
            self.0.push(bounds);
        }
    }
}

#[test]
fn complete_inventory_mounts_with_closed_menus_and_fits_both_documentation_widths() {
    for id in ["demo", "basic", "separator", "dropdown", "ellipsis", "link"] {
        let mut page = BreadcrumbPage::new(0);
        for (window_width, content_width) in [(1440., 638.), (390., 324.)] {
            page.window_width(window_width);
            let theme = Theme::light();
            let mut ui = Simulator::with_size(
                Settings {
                    text_size: 14.into(),
                    ..Settings::default()
                },
                Size::new(content_width, 300.),
                page.docs_preview(id, &theme),
            );
            frame(&mut ui);
            assert!(
                ui.find("Documentation").is_err(),
                "{id}: menu starts closed"
            );
            for label in ["Home", "Components", "Breadcrumb"] {
                let bounds = ui.find(label).expect("complete trail labels").bounds();
                assert!(bounds.x >= 0. && bounds.x + bounds.width <= content_width);
            }
            let first = ui.find("Home").unwrap().bounds();
            let current = ui.find("Breadcrumb").unwrap().bounds();
            let expected = if id == "dropdown" && window_width == 390. {
                26.
            } else {
                0.
            };
            // The hero's 28px trigger vertically centers its 20px labels.
            assert_eq!(current.y - first.y, expected, "{id} at {window_width}");
        }
    }
}

#[test]
fn dropdown_wraps_at_the_source_narrow_content_width_and_unwraps_on_resize() {
    let page = BreadcrumbPage::new(0);
    let mut ui = preview(&*page, "dropdown", 558.);
    frame(&mut ui);
    let first = ui.find("Home").unwrap().bounds();
    assert_eq!(ui.find("Breadcrumb").unwrap().bounds().y, first.y);
    ui = ui.resize(Size::new(260., 300.));
    frame(&mut ui);
    let current = ui.find("Breadcrumb").unwrap().bounds();
    assert_eq!(current.y - ui.find("Home").unwrap().bounds().y, 26.);
    assert!(current.x + current.width <= 260.);
    ui = ui.resize(Size::new(558., 300.));
    frame(&mut ui);
    assert_eq!(ui.find("Breadcrumb").unwrap().bounds().y, first.y);
}

#[test]
fn both_menus_open_navigate_select_escape_and_reopen_in_the_installed_previews() {
    for id in ["demo", "dropdown"] {
        let page = BreadcrumbPage::new(0);
        let mut ui = preview(&*page, id, 558.);
        frame(&mut ui);
        assert!(ui.find("Documentation").is_err());
        let home = ui.find("Home").unwrap().bounds();
        let target = if id == "dropdown" {
            ui.find("Components").unwrap().bounds().center()
        } else {
            Point::new(home.x + home.width + 26. + 14., home.center_y())
        };
        ui.point_at(target);
        ui.simulate(simulator::click());
        ui = ui.rebuild(page.preview(id));
        frame(&mut ui);
        for label in ["Documentation", "Themes", "GitHub"] {
            ui.find(label).expect("menu destination");
        }
        for (key, label) in [
            (Named::End, "GitHub"),
            (Named::Home, "Documentation"),
            (Named::ArrowDown, "Themes"),
            (Named::ArrowUp, "Documentation"),
        ] {
            ui.tap_key(key);
            // Anchored keeps the trigger's focus for restoration. Inspect the
            // overlay target too instead of selecting the first focused node.
            let mut focus = Focused::default();
            ui.operate(&mut focus);
            let item = ui.find(label).unwrap().bounds();
            assert!(
                focus.0.iter().any(|bounds| bounds.contains(item.center())),
                "{id}: {label} receives menu focus"
            );
        }
        ui.tap_key(Named::Escape);
        ui = ui.rebuild(page.preview(id));
        frame(&mut ui);
        assert!(ui.find("Documentation").is_err());
        assert!(
            ui.find(selector::is_focused())
                .expect("trigger regains focus")
                .bounds()
                .contains(target)
        );
        ui.point_at(Point::new(550., 290.));
        ui.tap_key(Named::Enter);
        ui = ui.rebuild(page.preview(id));
        frame(&mut ui);
        ui.find("Documentation").expect("Enter reopens menu");
        ui.tap_key(Named::End);
        ui.tap_key(Named::Enter);
        ui = ui.rebuild(page.preview(id));
        frame(&mut ui);
        assert!(ui.find("Documentation").is_err(), "select closes {id}");
        ui.tap_key(Named::Space);
        ui = ui.rebuild(page.preview(id));
        frame(&mut ui);
        ui.find("Documentation").expect("Space reopens menu");
        ui.point_at(Point::new(550., 290.));
        ui.simulate(simulator::click());
        ui = ui.rebuild(page.preview(id));
        frame(&mut ui);
        assert!(
            ui.find("Documentation").is_err(),
            "outside click closes {id}"
        );
    }
}

#[test]
fn links_and_menu_trigger_are_keyboard_reachable_but_current_page_is_not() {
    let page = BreadcrumbPage::new(0);
    let mut ui = preview(&*page, "dropdown", 558.);
    ui.point_at(Point::new(550., 290.));
    next(&mut ui);
    frame(&mut ui);
    assert!(
        ui.find(selector::is_focused())
            .unwrap()
            .bounds()
            .contains(ui.find("Home").unwrap().bounds().center())
    );
    next(&mut ui);
    frame(&mut ui);
    assert!(
        ui.find(selector::is_focused())
            .unwrap()
            .bounds()
            .contains(ui.find("Components").unwrap().bounds().center())
    );
    ui.tap_key(Named::Enter);
    ui = ui.rebuild(page.preview("dropdown"));
    frame(&mut ui);
    ui.find("Documentation").expect("keyboard-opened dropdown");
}
