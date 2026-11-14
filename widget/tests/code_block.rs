//! Code Block's viewport geometry and keyboard navigation, using public APIs.
use iced_core::keyboard::key::Named;
use iced_core::widget::Id;
use iced_core::widget::operation::{Operation, Outcome, Scrollable, focusable};
use iced_core::{Element, Event, Point, Rectangle, Size, Vector, mouse};
use iced_test::Simulator;
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::code_block::LineStyle;
use icedxdy_widget::{CodeBlock, Slot, button, container};

type Ui = Simulator<'static, (), Theme, Renderer>;

fn block(
    cap: Option<f32>,
    exact: Option<f32>,
    collapsed: bool,
) -> Element<'static, (), Theme, Renderer> {
    let mut block = CodeBlock::lines([(
        LineStyle::Default,
        Slot::from(container(Space::new().width(1000).height(500))),
    )])
    .id("code")
    .collapsed(collapsed);
    if let Some(cap) = cap {
        block = block.max_height(cap);
    }
    if let Some(height) = exact {
        block = block.height(height);
    }
    container(block).width(448).into()
}

#[derive(Default)]
struct Metrics {
    bounds: Rectangle,
    content: Size,
    offset: Vector,
}
impl Operation for Metrics {
    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Operation)) {
        f(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        offset: Vector,
        _: &mut dyn Scrollable,
    ) {
        *self = Self {
            bounds,
            content,
            offset,
        };
    }
}
fn metrics(ui: &mut Ui) -> Metrics {
    ui.draw(&Theme::light());
    let mut op = Metrics::default();
    ui.operate(&mut op);
    op
}
fn next(ui: &mut Ui) {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
}
fn ui(content: Element<'static, (), Theme, Renderer>, width: f32) -> Ui {
    Simulator::with_size(Default::default(), Size::new(width, 1000.), content)
}

#[test]
fn cap_preserves_full_source_and_shrinks_with_available_width() {
    let mut view = ui(block(Some(192.), None, false), 600.);
    let m = metrics(&mut view);
    assert_eq!(m.bounds.height, 192.);
    assert_eq!(m.bounds.width, 446.);
    assert_eq!(m.content.height, 524.);
    assert!(m.content.width >= 1024.);
    view = view.resize(Size::new(260., 1000.));
    let m = metrics(&mut view);
    assert_eq!(m.bounds.width, 258.);
    assert_eq!(m.bounds.height, 192.);
    assert_eq!(m.content.height, 524.);
}

#[test]
fn maximum_height_never_stretches_short_output_but_exact_height_does() {
    let block = || {
        CodeBlock::lines([(
            LineStyle::Default,
            Slot::from(container(Space::new().width(20).height(30))),
        )])
    };
    let mut capped = ui(container(block().max_height(192.)).width(448).into(), 600.);
    assert_eq!(metrics(&mut capped).bounds.height, 54.);
    let mut fixed = ui(container(block().height(192.)).width(448).into(), 600.);
    assert_eq!(metrics(&mut fixed).bounds.height, 192.);
}

#[test]
fn focused_content_scrolls_by_keyboard_and_clamps_at_edges() {
    let mut view = ui(block(Some(192.), None, false), 600.);
    view.draw(&Theme::light());
    next(&mut view);
    view.tap_key(Named::ArrowRight);
    view.tap_key(Named::PageDown);
    let m = metrics(&mut view);
    assert_eq!(m.offset.x, 40.);
    assert!((m.offset.y - 172.8).abs() < 0.01);
    view.tap_key(Named::End);
    let m = metrics(&mut view);
    assert_eq!(m.offset.y, m.content.height - m.bounds.height);
    view.tap_key(Named::Home);
    assert_eq!(metrics(&mut view).offset, Vector::ZERO);
    view.tap_key(Named::ArrowLeft);
    view.tap_key(Named::ArrowUp);
    assert_eq!(metrics(&mut view).offset, Vector::ZERO);
}

#[test]
fn collapsed_content_keeps_complete_source_and_does_not_scroll_vertically() {
    let mut view = ui(block(Some(256.), None, true), 600.);
    view.draw(&Theme::light());
    next(&mut view);
    view.tap_key(Named::PageDown);
    view.point_at(Point::new(40., 80.));
    view.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0., y: -4. },
    })]);
    let m = metrics(&mut view);
    assert_eq!(m.offset.y, 0.);
    assert_eq!(m.content.height, 524.);
    view = view.rebuild(block(Some(384.), None, false));
    view.tap_key(Named::End);
    assert_eq!(metrics(&mut view).offset.y, 140.);
}

#[test]
fn floating_actions_precede_content_and_survive_rebuild() {
    let block = || icedxdy_widget::code_block("output").actions(button("Copy").on_press(()));
    let mut view = ui(container(block()).width(448).into(), 600.);
    view.point_at(Point::new(550., 900.));
    view.draw(&Theme::light());
    next(&mut view);
    view.draw(&Theme::light());
    view.tap_key(Named::Enter);
    assert_eq!(view.drain().count(), 1);
    let rebuilt: Element<'_, (), Theme, Renderer> = container(block()).width(448).into();
    view = view.rebuild(rebuilt);
    view.draw(&Theme::light());
    view.tap_key(Named::Space);
    assert_eq!(view.drain().count(), 1);
}
