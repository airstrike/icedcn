//! Installed Checkbox examples: state coupling and constrained layout regressions.
use std::any::Any;
use std::time::Duration;

use browser::Message;
use browser::component::checkbox::CheckboxPage;
use browser::page::Page;
use iced::{Event, Font, Length, Point, Rectangle, Size, Vector, mouse, time::Instant, window};
use iced_test::core::widget::operation::Scrollable;
use iced_test::core::widget::{Id, Operation};
use iced_test::core::{Settings, Widget};
use iced_test::simulator::{self, Simulator};
use icedxdy::Theme;
use icedxdy::widget::operation::control;

type Preview<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

#[derive(Default, Debug)]
struct Controls(Vec<(Rectangle, bool)>, Vector);

impl Operation for Controls {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        let mut read = control::checked();
        read.custom(id, bounds, state);
        if let Some(checked) = read.get() {
            self.0.push((bounds, checked));
        }
    }

    fn scrollable(
        &mut self,
        _: Option<&Id>,
        _: Rectangle,
        _: Size,
        translation: Vector,
        _: &mut dyn Scrollable,
    ) {
        self.1 = translation;
    }
}

fn controls(ui: &mut Preview<'_>) -> Controls {
    let mut controls = Controls::default();
    ui.operate(&mut controls);
    controls
}

fn preview<'a>(page: &'a dyn Page, id: &str, width: f32) -> Preview<'a> {
    let settings = Settings {
        font: Font::new("Geist"),
        ..Settings::default()
    };
    let element = page.docs_preview(id, &Theme::light());
    assert_eq!(
        element.size().height,
        Length::Fixed(if id == "demo" { 320.0 } else { 288.0 })
    );
    Simulator::with_size(settings, Size::new(width, 600.0), element)
}

fn click(ui: &mut Preview<'_>, point: Point) {
    ui.point_at(point);
    ui.simulate(simulator::click());
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
}

#[test]
fn all_installed_defaults_and_control_bounds_survive_two_content_widths() {
    for (window, width) in [(1440.0, 634.0), (390.0, 324.0)] {
        let mut page = CheckboxPage::new(0);
        page.window_width(window);
        for (id, expected) in [
            ("demo", vec![false, true, false, false]),
            ("checked-state", vec![true]),
            ("invalid-state", vec![false]),
            ("basic", vec![false]),
            ("description", vec![true]),
            ("disabled", vec![false]),
            ("group", vec![true, true, false, false]),
            ("table", vec![false, true, false, false, false]),
        ] {
            let mut ui = preview(&*page, id, width);
            let observed = controls(&mut ui);
            println!("window={window} content={width} id={id} {observed:?}");
            assert_eq!(
                observed
                    .0
                    .iter()
                    .map(|(_, checked)| *checked)
                    .collect::<Vec<_>>(),
                expected,
                "{id}"
            );
            for (bounds, _) in observed.0 {
                assert_eq!(bounds.width, 16.0, "{id}");
                assert_eq!(bounds.height, 16.0, "{id}");
                assert!(
                    bounds.x >= if id == "table" { 16.0 } else { 40.0 }
                        && bounds.x + bounds.width <= width,
                    "{id}: {bounds:?}"
                );
                assert!(
                    bounds.y >= 0.0 && bounds.y + bounds.height <= 320.0,
                    "{id}: {bounds:?}"
                );
            }
        }
    }
}

#[test]
fn group_label_changes_only_its_own_checkbox() {
    for (window, width) in [(1440.0, 634.0), (390.0, 324.0)] {
        let mut page = CheckboxPage::new(0);
        page.window_width(window);
        let mut ui = preview(&*page, "group", width);
        let (bounds, _) = controls(&mut ui).0[2];
        click(&mut ui, Point::new(bounds.x + 70.0, bounds.center_y()));
        assert_eq!(
            controls(&mut ui)
                .0
                .iter()
                .map(|(_, value)| *value)
                .collect::<Vec<_>>(),
            [true, true, true, false]
        );
        click(&mut ui, Point::new(bounds.x + 70.0, bounds.center_y()));
        assert_eq!(
            controls(&mut ui)
                .0
                .iter()
                .map(|(_, value)| *value)
                .collect::<Vec<_>>(),
            [true, true, false, false]
        );
    }
}

#[test]
fn table_header_and_rows_apply_complete_selection_at_both_widths() {
    for (window, width) in [(1440.0, 634.0), (390.0, 324.0)] {
        let mut page = CheckboxPage::new(0);
        page.window_width(window);
        let mut ui = preview(&*page, "table", width);
        let positions = controls(&mut ui).0;
        click(&mut ui, positions[0].0.center());
        assert!(controls(&mut ui).0.iter().all(|(_, value)| *value));
        click(&mut ui, positions[2].0.center());
        assert_eq!(
            controls(&mut ui)
                .0
                .iter()
                .map(|(_, value)| *value)
                .collect::<Vec<_>>(),
            [false, true, false, true, true]
        );
        click(&mut ui, positions[0].0.center());
        assert!(controls(&mut ui).0.iter().all(|(_, value)| *value));
        click(&mut ui, positions[0].0.center());
        assert!(controls(&mut ui).0.iter().all(|(_, value)| !value));
        let positions = controls(&mut ui).0;
        for pair in positions[1..].windows(2) {
            assert!(
                (pair[1].0.y - pair[0].0.y - 37.0).abs() < 1.0,
                "no narrow row wrapping: {positions:?}"
            );
        }
        if width < 400.0 {
            ui.point_at(Point::new(width * 0.5, 140.0));
            ui.scroll(mouse::ScrollDelta::Pixels { x: -500.0, y: 0.0 });
            ui.simulate([Event::Window(window::Event::RedrawRequested(
                Instant::now() + Duration::from_secs(1),
            ))]);
            let role = ui
                .find("Editor")
                .expect("last column is reachable")
                .bounds();
            let role = role - controls(&mut ui).1;
            assert!(
                role.x >= 16.0 && role.x + role.width <= width - 16.0,
                "{role:?}"
            );
        }
    }
}
