//! A colspan covers precisely the same tracks measured across header and body.
use iced_core::Length::Fill;
use iced_core::widget::{Id, Operation};
use iced_core::{Element, Rectangle};
use iced_test::simulator::simulator;
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::table::Track;
use icedxdy_widget::{container, table};

#[derive(Default)]
struct Bounds {
    auto: Option<Rectangle>,
    second: Option<Rectangle>,
    end: Option<Rectangle>,
    span: Option<Rectangle>,
}
impl Operation for Bounds {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn container(&mut self, id: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
        for (name, out) in [
            ("auto", &mut self.auto),
            ("second", &mut self.second),
            ("end", &mut self.end),
            ("span", &mut self.span),
        ] {
            if id == Some(&Id::new(name)) {
                *out = Some(bounds);
            }
        }
    }
}
#[test]
fn shares_intrinsic_and_spanning_columns() {
    let mark = |id| container(Space::new()).height(20).width(Fill).id(id);
    let grid = table::grid([Track::Auto, Track::Flex(1), Track::Fixed(56.0)])
        .header(table::header([table::row([
            table::head(mark("auto")),
            table::head(mark("second")),
            table::head(mark("end")),
        ])]))
        .body(table::body([table::row([
            table::cell(Space::new().width(72).height(10)),
            table::cell(""),
            table::cell(""),
        ])]))
        .footer(table::footer([table::row([
            table::cell(mark("span")).span(2),
            table::cell(""),
        ])]));
    let element: Element<'_, (), Theme, Renderer> = container(grid).width(320).into();
    let mut ui = simulator(element);
    let mut b = Bounds::default();
    ui.operate(&mut b);
    let (auto, second, end, span) = (
        b.auto.unwrap(),
        b.second.unwrap(),
        b.end.unwrap(),
        b.span.unwrap(),
    );
    assert_eq!(auto.width, 72.0);
    assert_eq!(second.x, auto.x + auto.width + 16.0);
    assert_eq!(span.x, auto.x);
    assert_eq!(span.width, end.x - span.x - 16.0);
}
