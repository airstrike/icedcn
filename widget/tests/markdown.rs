//! Markdown tables must share the surrounding page's gesture ownership.
#![cfg(feature = "markdown")]

use iced_core::time::Instant;
use iced_core::widget::{Id, Operation, operation};
use iced_core::{Element, Event, Point, Rectangle, Size, Vector, mouse, touch, window};
use iced_test::{Simulator, simulator};
use iced_widget::{Renderer, Space, Widget};
use icedcn_core::Theme;
use icedcn_widget::{column, markdown, scrollable};

struct Cells;

impl<'a> markdown::Viewer<'a, (), Theme, Renderer> for Cells {
    fn on_link_click(_: markdown::Uri) {}

    fn paragraph(
        &self,
        _: markdown::Settings,
        _: &markdown::Text,
    ) -> Element<'a, (), Theme, Renderer> {
        // The null renderer cannot measure text; give real table cells fixed geometry.
        Space::new().width(240).height(30).boxed()
    }
}

fn article(items: &[markdown::Item]) -> Simulator<'_, (), Theme, Renderer> {
    let table = markdown::view_with(items, iced_core::Theme::Light, &Cells);
    simulator(Element::from(
        scrollable(column![table, Space::new().height(800)])
            .id("article")
            .width(160)
            .height(150),
    ))
}

#[derive(Default)]
struct Offsets {
    article: f32,
    table: Vector,
}

impl Operation for Offsets {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn scrollable(
        &mut self,
        id: Option<&Id>,
        _: Rectangle,
        _: Size,
        translation: Vector,
        _: &mut dyn operation::Scrollable,
    ) {
        if id == Some(&Id::new("article")) {
            self.article = translation.y;
        } else {
            self.table = translation;
        }
    }
}

fn offsets(ui: &mut Simulator<'_, (), Theme, Renderer>) -> Offsets {
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    let mut offsets = Offsets::default();
    ui.operate(&mut offsets);
    offsets
}

#[test]
fn table_wheel_scrolls_only_its_axis_and_passes_vertical_input_to_the_page() {
    let items: Vec<_> =
        markdown::parse("| Prop | Type |\n| --- | --- |\n| value | bool |").collect();
    let mut ui = article(&items);
    ui.point_at(Point::new(80., 30.));
    for (x, y, table, page) in [
        (-16., -2., 16., 0.),
        (-1., -12., 16., 12.),
        (-8., 0., 24., 12.),
    ] {
        let _ = ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Pixels { x, y },
        })]);
        let offset = offsets(&mut ui);
        assert_eq!(offset.table, Vector::new(table, 0.));
        assert_eq!(offset.article, page);
    }
}

#[test]
fn table_touch_drag_routes_to_one_axis_without_blocking_the_page() {
    let items: Vec<_> =
        markdown::parse("| Prop | Type |\n| --- | --- |\n| value | bool |").collect();
    for (end, table, page) in [
        (Point::new(79., 8.), 0., 22.),
        (Point::new(50., 28.), 30., 0.),
    ] {
        let mut ui = article(&items);
        let start = Point::new(80., 30.);
        let finger = touch::Finger(1);
        ui.point_at(start);
        let _ = ui.simulate([Event::Touch(touch::Event::FingerPressed {
            id: finger,
            position: start,
            timestamp: None,
        })]);
        ui.point_at(end);
        let _ = ui.simulate([Event::Touch(touch::Event::FingerMoved {
            id: finger,
            position: end,
            timestamp: None,
        })]);
        let offset = offsets(&mut ui);
        assert_eq!(offset.table, Vector::new(table, 0.));
        assert_eq!(offset.article, page);
    }
}
