//! Code Block's viewport geometry and keyboard navigation, using public APIs.
use iced_core::keyboard::key::Named;
use iced_core::widget::Id;
use iced_core::widget::operation::{Operation, Outcome, Scrollable, focusable};
use iced_core::{Element, Event, Point, Rectangle, Size, Vector, mouse};
use iced_test::{Simulator, simulator};
use iced_widget::{Renderer, Space};
use icedcn_core::Theme;
use icedcn_widget::code_block::{self, CodeBlock, Highlighted, Removed, line};
use icedcn_widget::{Slot, button, container};

type Ui = Simulator<'static, (), Theme, Renderer>;

fn block(
    cap: Option<f32>,
    exact: Option<f32>,
    collapsed: bool,
) -> Element<'static, (), Theme, Renderer> {
    let mut content =
        code_block::content(vec![line(container(Space::new().width(1000).height(500)))])
            .id("code")
            .collapsed(collapsed);
    if let Some(cap) = cap {
        content = content.max_height(cap);
    }
    if let Some(height) = exact {
        content = content.height(height);
    }
    container(code_block::code_block(content)).width(448).into()
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
    let content = || code_block::content(vec![line(container(Space::new().width(20).height(30)))]);
    let block = |content| container(code_block::code_block(content)).width(448).into();
    let mut capped = ui(block(content().max_height(192)), 600.);
    assert_eq!(metrics(&mut capped).bounds.height, 54.);
    let mut fixed = ui(block(content().height(192)), 600.);
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
    let block =
        || icedcn_widget::code_block![code_block::actions(button("Copy").on_press(())), "output"];
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

#[test]
fn header_actions_stay_at_the_end_at_both_widths() {
    #[derive(Default)]
    struct Parts {
        title: Rectangle,
        actions: Rectangle,
    }
    impl Operation for Parts {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn container(&mut self, id: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
            if id == Some(&Id::from("title")) {
                self.title = bounds;
            } else if id == Some(&Id::from("actions")) {
                self.actions = bounds;
            }
        }
    }
    for compact in [false, true] {
        for width in [512., 260.] {
            for title_width in [40., 800.] {
                let title = container(Space::new().width(title_width).height(20)).id("title");
                let action = container(button(Space::new().width(68).height(18)).on_press(()));
                let block = icedcn_widget::code_block![
                    code_block::header![
                        code_block::title(Slot::from(title)),
                        code_block::actions![action.id("actions")],
                    ],
                    "output",
                ]
                .size(if compact {
                    icedcn_widget::Size::Sm
                } else {
                    icedcn_widget::Size::Default
                });
                let mut view = ui(container(block).width(width).into(), width);
                view.draw(&Theme::light());
                let mut parts = Parts::default();
                view.operate(&mut parts);
                let padding = if compact { 10. } else { 12. };
                assert_eq!(parts.actions.x + parts.actions.width, width - padding + 5.);
                assert!(parts.title.x + parts.title.width + 8. <= parts.actions.x);
                assert!(parts.actions.height > 0.);
                view.point_at(parts.actions.center());
                view.simulate(simulator::click());
                assert_eq!(view.drain().count(), 1, "header action remains clickable");
            }
        }
    }
}

#[cfg(feature = "highlighter")]
#[test]
fn highlighted_patch_rebuilds_with_incoming_source_and_preserves_rows() {
    use iced_core::widget::operation::Text;
    use icedcn_widget::code_block::code_block;

    #[derive(Default)]
    struct Rows(Vec<String>);
    impl Operation for Rows {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn text(&mut self, _: Option<&Id>, _: Rectangle, text: &mut dyn Text) {
            self.0.push(text.text().to_string());
        }
    }
    fn rendered(view: &mut Ui, theme: &Theme) -> Vec<String> {
        view.draw(theme);
        let mut text = Rows::default();
        view.operate(&mut text);
        text.0
    }
    let listing = |source: &str| {
        code_block(
            code_block::content(source.to_owned())
                .language("rust")
                .line_style(2, Highlighted)
                .line_style(3, Removed),
        )
        .into()
    };
    let mut view = ui(listing("let name = \"café\";\nna"), 400.);
    assert!(rendered(&mut view, &Theme::light()).contains(&"na".to_owned()));
    let source = "let name = \"café\";\nname.len()\n0";
    view = view.rebuild(listing(source));
    let rows = rendered(&mut view, &Theme::dark());
    for line in source.lines() {
        assert!(
            rows.iter().any(|text| text == line),
            "missing {line:?} in {rows:?}"
        );
    }
}

#[test]
fn header_accepts_borrowed_source_and_messages() {
    fn view<'a, Message>(source: &'a str) -> CodeBlock<'a, Message> {
        icedcn_widget::code_block![
            code_block::header![
                "Terminal",
                code_block::actions![code_block::copy_button(source)]
            ],
            source,
        ]
    }

    fn mount<'a>(source: &'a mut str) -> Element<'a, &'a mut str, Theme, Renderer> {
        view(source).into()
    }

    let mut source = String::from("+fn main() {}");
    drop(mount(&mut source));
}
