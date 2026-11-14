//! Focused regressions for the repaired width, clipping and selection paths.
use browser::component::code_block::CodeBlockPage;
use iced::widget::Id;
use iced::{Element, Point, Rectangle, Size, Vector};
use iced_test::Simulator;
use iced_test::core::text::Target;
use iced_test::core::widget::operation::{self, Operation, Outcome, Scrollable, text};
use icedxdy::Theme;
use icedxdy::widget::code_block::{CodeBlock, LineStyle};
use icedxdy::widget::{code_block, container, rich_text, span};

#[derive(Default)]
struct Viewports(Vec<(Rectangle, Size)>);
impl Operation for Viewports {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        _: Vector,
        _: &mut dyn Scrollable,
    ) {
        self.0.push((bounds, content));
    }
}

#[test]
fn streaming_and_markdown_use_the_available_preview_width() {
    let page = CodeBlockPage::new(0);
    let theme = Theme::light();
    for id in ["streaming", "markdown"] {
        for width in [558.0_f32, 260.0] {
            let mut ui = Simulator::with_size(
                Default::default(),
                Size::new(width, 1600.),
                page.docs_preview(id, &theme),
            );
            let mut viewports = Viewports::default();
            ui.operate(&mut viewports);
            assert!(!viewports.0.is_empty());
            for (bounds, _) in viewports.0 {
                assert_eq!(bounds.width, width.min(448.0) - 2.0, "{id}");
            }
        }
    }
}

#[test]
fn demo_and_wrap_keep_their_bounds_at_two_widths_without_truncating_source() {
    let page = CodeBlockPage::new(0);
    for width in [558., 260.] {
        let theme = Theme::light();
        let mut ui = Simulator::with_size(
            Default::default(),
            Size::new(width, 1600.),
            page.docs_preview("demo", &theme),
        );
        ui.draw(&theme);
        let mut viewports = Viewports::default();
        ui.operate(&mut viewports);
        let (bounds, content) = viewports.0[0];
        assert_eq!(bounds.width, width.min(512.) - 2.);
        assert!(
            content.width > bounds.width,
            "unwrapped source retains long lines"
        );
        assert_eq!(
            content.height,
            13. * 22.75 + 20.,
            "13 lines at an absolute 22.75px, plus padding"
        );
        let mut ui = Simulator::with_size(
            Default::default(),
            Size::new(width, 1600.),
            page.docs_preview("wrap", &theme),
        );
        ui.draw(&theme);
        let mut viewports = Viewports::default();
        ui.operate(&mut viewports);
        assert_eq!(viewports.0.len(), 2);
        let (unwrapped, long) = viewports.0[0];
        let (wrapped, reflowed) = viewports.0[1];
        assert_eq!(unwrapped.width, width.min(448.) - 2.);
        assert!(long.width > unwrapped.width);
        assert!(reflowed.width <= wrapped.width + 0.1);
        assert!(wrapped.height > unwrapped.height);
    }
}

fn selected(element: Element<'static, (), Theme>) -> String {
    let mut ui = Simulator::with_size(Default::default(), Size::new(448., 500.), element);
    ui.draw(&Theme::light());
    ui.operate(&mut text::select_all());
    let mut copy = text::copy();
    ui.operate(&mut operation::black_box(&mut copy));
    match copy.finish() {
        Outcome::Some(value) => value,
        _ => panic!("selection copy"),
    }
}

#[test]
fn selection_preserves_blank_lines_and_excludes_numbers_and_diff_markers() {
    assert_eq!(
        selected(
            container(code_block("first\n\nlast").start_line(995))
                .width(448)
                .into()
        ),
        "first\n\nlast"
    );
    let block = CodeBlock::lines([
        (LineStyle::Removed, rich_text([span("old")]).into()),
        (LineStyle::Default, rich_text([span("")]).into()),
        (LineStyle::Added, rich_text([span("new")]).into()),
    ])
    .start_line(1);
    assert_eq!(selected(container(block).width(448).into()), "old\n\nnew");
}

#[test]
fn selecting_one_line_does_not_copy_adjacent_blank_rows() {
    let element: Element<'_, (), Theme> = container(code_block("first\n\nlast").start_line(995))
        .width(448)
        .into();
    let mut ui = Simulator::with_size(Default::default(), Size::new(448., 500.), element);
    ui.draw(&Theme::light());
    let line = ui.find("first").expect("first code line").bounds();
    ui.operate(&mut text::select(
        Point::new(line.x - 1., line.center_y()),
        Point::new(line.x + line.width + 1., line.center_y()),
        Target::Character,
    ));
    let mut copy = text::copy();
    ui.operate(&mut operation::black_box(&mut copy));
    assert!(matches!(copy.finish(), Outcome::Some(value) if value == "first"));
}

#[test]
fn show_more_state_stays_local_across_theme_and_width_rebuilds() {
    let page = CodeBlockPage::new(0);
    let mut ui = Simulator::with_size(
        Default::default(),
        Size::new(558., 1800.),
        page.docs_preview("show-more", &Theme::light()),
    );
    ui.draw(&Theme::light());
    ui.click("Show more").expect("mounted footer");
    ui.draw(&Theme::light());
    assert!(ui.find("Show less").is_ok());
    assert_eq!(
        ui.drain().count(),
        0,
        "local expansion publishes no page message"
    );
    let mut bounds = Viewports::default();
    ui.operate(&mut bounds);
    assert!(bounds.0[0].0.height > 256.);
    ui = ui
        .rebuild(page.docs_preview("show-more", &Theme::dark()))
        .resize(Size::new(260., 1800.));
    ui.draw(&Theme::dark());
    assert!(ui.find("Show less").is_ok());
    let mut expanded = Viewports::default();
    ui.operate(&mut expanded);
    assert_eq!(expanded.0[0].0.height, 38. * 22.75 + 16.);
    ui.click("Show less").expect("footer after rebuild");
    ui.draw(&Theme::dark());
    let mut bounds = Viewports::default();
    ui.operate(&mut bounds);
    assert_eq!(bounds.0[0].0.height, 256.);
    assert_eq!(bounds.0[0].0.width, 258.);
    assert!(ui.find("Show more").is_ok());
    assert_eq!(ui.drain().count(), 0);
}
