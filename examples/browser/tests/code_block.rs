//! Focused regressions for the repaired width, clipping and selection paths.
use browser::component::code_block::CodeBlockPage;
use iced::keyboard::{self, key::Named};
use iced::mouse::ScrollDelta;
use iced::widget::Id;
use iced::{Element, Event, Point, Rectangle, Size, Vector};
use iced_test::core::text::Target;
use iced_test::core::widget::operation::{self, Operation, Outcome, Scrollable, TextInput, text};
use iced_test::{Simulator, simulator};
use icedcn::Theme;
use icedcn::widget::code_block::{Added, CodeBlock, Removed, line};
use icedcn::widget::{code_block, column, container, rich_text, span};

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

fn numbered(source: &str) -> code_block::Content<'_, ()> {
    code_block::content(source)
        .line_numbers(true)
        .start_line(995)
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
            container(code_block(numbered("first\n\nlast")))
                .width(448)
                .into()
        ),
        "first\n\nlast"
    );
    let block = code_block(
        code_block::content(vec![
            line(rich_text([span("old")])).style(Removed),
            line(rich_text([span("")])),
            line(rich_text([span("new")])).style(Added),
        ])
        .line_numbers(true),
    );
    assert_eq!(selected(container(block).width(448).into()), "old\n\nnew");
}

#[test]
fn selecting_one_line_does_not_copy_adjacent_blank_rows() {
    let element: Element<'_, (), Theme> = container(code_block(numbered("first\n\nlast")))
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
    for (example, open_height) in [("show-more", 38. * 22.75 + 16.), ("show-more-scroll", 384.)] {
        let mut ui = Simulator::with_size(
            Default::default(),
            Size::new(558., 1800.),
            page.docs_preview(example, &Theme::light()),
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
            .rebuild(page.docs_preview(example, &Theme::dark()))
            .resize(Size::new(260., 1800.));
        ui.draw(&Theme::dark());
        assert!(ui.find("Show less").is_ok());
        let mut expanded = Viewports::default();
        ui.operate(&mut expanded);
        assert_eq!(expanded.0[0].0.height, open_height);
        assert_eq!(expanded.0[0].1.height, 38. * 22.75 + 16.);
        ui.click("Show less").expect("footer after rebuild");
        ui.draw(&Theme::dark());
        let mut bounds = Viewports::default();
        ui.operate(&mut bounds);
        assert_eq!(bounds.0[0].0.height, 256.);
        assert_eq!(bounds.0[0].0.width, 258.);
        assert!(ui.find("Show more").is_ok());
        assert_eq!(ui.drain().count(), 0);
    }
}

#[test]
fn ansi_matches_explicit_terminal_spans_at_both_scales_and_themes() {
    use iced::widget::text::{LineHeight, Wrapping};
    use iced::{Fill, Font, Renderer, mouse};
    use iced_test::core::renderer::{Headless, Style};
    use iced_test::futures::futures::executor::block_on;
    use iced_test::runtime::UserInterface;

    let mut renderer =
        block_on(Renderer::new(Default::default(), Some("wgpu"))).expect("headless renderer");
    for theme in [Theme::light(), Theme::dark()] {
        for scale in [1, 2] {
            let screenshot = |renderer: &mut Renderer, block: CodeBlock<'_, ()>| {
                let element: Element<'_, (), Theme> = block.into();
                let mut ui = UserInterface::build(
                    element,
                    Size::new(160., 104.),
                    Default::default(),
                    renderer,
                );
                ui.draw(
                    renderer,
                    &theme,
                    &Style::default(),
                    mouse::Cursor::Unavailable,
                );
                renderer.screenshot(
                    Size::new(160 * scale, 104 * scale),
                    scale as f32,
                    theme.neutral.base.into(),
                )
            };
            let color = |light, dark, alpha| {
                let rgb = if theme.neutral.is_dark { dark } else { light };
                icedcn::Color::from(iced::color!(
                    (rgb >> 16) as u8,
                    (rgb >> 8) as u8,
                    rgb as u8,
                    alpha
                ))
            };
            let row = |spans| {
                let text = rich_text(spans)
                    .font(Font::MONOSPACE)
                    .size(14)
                    .line_height(LineHeight::Absolute(22.75.into()))
                    .wrapping(Wrapping::WordOrGlyph)
                    .width(Fill);
                line(text)
            };
            let normal = color(0x24292e, 0xe1e4e8, 1.);
            let red = color(0xd73a49, 0xea4a5a, 1.);
            let green = color(0x28a745, 0x34d058, 1.);
            let dim = color(0x24292e, 0xe1e4e8, 128. / 255.);
            let reference = code_block(
                code_block::content(vec![
                    row(vec![
                        span("Cargo ").color(normal),
                        span("FAILED").color(red),
                    ]),
                    row(vec![span("")]),
                    row(vec![span("tests ").color(dim), span("ok").color(green)]),
                ])
                .wrap(true),
            );
            let output = code_block(
                code_block::content("Cargo \x1b[31mFAILED\n\n\x1b[2mtests \x1b[22;32mok")
                    .language("ansi")
                    .wrap(true),
            );
            let expected = screenshot(&mut renderer, reference);
            let actual = screenshot(&mut renderer, output);
            let different = expected
                .iter()
                .zip(actual)
                .filter(|(l, r)| **l != *r)
                .count();
            assert_eq!(
                different, 0,
                "terminal text pixels at {scale}×, dark={}",
                theme.neutral.is_dark
            );
        }
    }
}

#[derive(Default)]
struct Editors(Vec<(Rectangle, String)>);
impl Operation for Editors {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, input: &mut dyn TextInput) {
        self.0.push((bounds, input.text().into_owned()));
    }
}

fn editor<'a, M: 'a>(ui: &mut Simulator<'a, M, Theme>) -> (Rectangle, String) {
    let mut editors = Editors::default();
    ui.operate(&mut editors);
    assert_eq!(editors.0.len(), 1);
    editors.0.pop().unwrap()
}

fn editable(value: &str) -> Element<'static, String, Theme> {
    code_block![
        code_block::content(value.to_owned())
            .language("rust")
            .line_numbers(true)
            .on_input(|source| source)
    ]
    .into()
}

#[test]
fn editable_typing_survives_a_rebuild_with_the_previous_value() {
    for initial in ["", "fn main() {}\n"] {
        let mut value = initial.to_owned();
        let mut ui = Simulator::new(editable(&value));
        let (bounds, _) = editor(&mut ui);
        ui.point_at(Point::new(bounds.x + 2., bounds.y + 4.));
        ui.simulate(simulator::click());
        for character in ["p", "u", "b"] {
            ui.typewrite(character);
            // iced can rebuild the view before the app handles the edit, so
            // the editor sees the old value once more.
            ui = ui.rebuild(editable(&value));
            if let Some(text) = ui.drain().last() {
                value = text;
            }
            ui = ui.rebuild(editable(&value));
        }
        assert_eq!(value, format!("pub{initial}"), "starting from {initial:?}");
    }
}

#[test]
fn editable_tab_indents_shift_tab_outdents_and_enter_keeps_the_indent() {
    let mut value = String::new();
    let mut ui = Simulator::new(editable(&value));
    let (bounds, _) = editor(&mut ui);
    ui.point_at(Point::new(bounds.x + 2., bounds.y + 4.));
    ui.simulate(simulator::click());
    let step = |ui: Simulator<'static, String, Theme>, value: &mut String| {
        let mut ui = ui;
        if let Some(text) = ui.drain().last() {
            *value = text;
        }
        ui.rebuild(editable(value))
    };
    ui.tap_key(Named::Tab);
    ui = step(ui, &mut value);
    assert_eq!(value, "    ", "Tab inserts four spaces");
    ui.typewrite("x");
    ui = step(ui, &mut value);
    ui.tap_key(Named::Enter);
    ui = step(ui, &mut value);
    ui.typewrite("y");
    ui = step(ui, &mut value);
    assert_eq!(value, "    x\n    y");
    ui.simulate(
        [simulator::press_key(Named::Tab, None)].map(|event| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                location,
                text,
                repeat,
                ..
            }) => Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                location,
                modifiers: keyboard::Modifiers::SHIFT,
                text,
                repeat,
            }),
            event => event,
        }),
    );
    let _ = step(ui, &mut value);
    assert_eq!(value, "    x\ny", "Shift+Tab outdents the line");
}

#[test]
fn editable_and_read_only_blocks_lay_out_to_the_same_size() {
    use icedcn::widget::text;

    let source = "fn main() {\n    println!(\"hi\");\n}\n";
    let read_only: Element<'static, String, Theme> = code_block![
        code_block::header![code_block::title("main.rs")],
        code_block::content(source)
            .language("rust")
            .line_numbers(true)
    ]
    .into();
    let editable: Element<'static, String, Theme> = code_block![
        code_block::header![code_block::title("main.rs")],
        code_block::content(source)
            .language("rust")
            .line_numbers(true)
            .on_input(|source| source)
    ]
    .into();
    let mut sizes = Vec::new();
    for block in [read_only, editable] {
        let view: Element<'static, String, Theme> = column![block, text("END")].into();
        let mut ui = Simulator::with_size(Default::default(), Size::new(480., 800.), view);
        sizes.push(ui.find("END").unwrap().bounds().y);
    }
    assert_eq!(sizes[0], sizes[1]);
}

#[derive(Default)]
struct Offsets(Vec<(Rectangle, Size, Vector)>);
impl Operation for Offsets {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        offset: Vector,
        _: &mut dyn Scrollable,
    ) {
        self.0.push((bounds, content, offset));
    }
}

/// Each scrollable's offset as of the next frame.
fn offsets<'a, M: 'a>(ui: &mut Simulator<'a, M, Theme>) -> Vec<(Rectangle, Size, Vector)> {
    ui.draw(&Theme::light());
    let mut offsets = Offsets::default();
    ui.operate(&mut offsets);
    offsets.0
}

fn wheel(x: f32, y: f32) -> ScrollDelta {
    ScrollDelta::Pixels { x, y }
}

const LONG: &str = "fn main() { println!(\"a line that is much longer than the code block is wide, so it has to scroll\"); }\n";

#[test]
fn a_long_line_in_an_unwrapped_editor_scrolls_on_a_horizontal_wheel_and_while_typing() {
    let block: Element<'static, (), Theme> = container(code_block![
        code_block::content("")
            .language("rust")
            .line_numbers(true)
            .default_value(LONG)
    ])
    .width(320)
    .into();
    let mut ui = Simulator::with_size(Default::default(), Size::new(320., 400.), block);
    let (bounds, content, offset) = offsets(&mut ui)[0];
    assert!(
        content.width > bounds.width + 200.,
        "the editor keeps its long line"
    );
    assert_eq!(offset, Vector::ZERO);
    let (line, _) = editor(&mut ui);
    ui.point_at(Point::new(
        bounds.x + 60.,
        line.center_y().min(bounds.y + 20.),
    ));
    ui.scroll(wheel(-90., 0.));
    assert_eq!(offsets(&mut ui)[0].2, Vector::new(90., 0.));
    ui.scroll(wheel(90., 0.));
    assert_eq!(offsets(&mut ui)[0].2, Vector::ZERO);

    ui.simulate(simulator::click());
    ui.tap_key(Named::End);
    let (bounds, content, offset) = offsets(&mut ui)[0];
    assert_eq!(
        offset.x,
        content.width - bounds.width,
        "End reveals the caret"
    );
    ui.typewrite("// more");
    let (bounds, grown, offset) = offsets(&mut ui)[0];
    assert!(grown.width > content.width, "the line grows as it is typed");
    assert_eq!(
        offset.x,
        grown.width - bounds.width,
        "the caret stays in view"
    );
    ui.tap_key(Named::Home);
    assert_eq!(offsets(&mut ui)[0].2.x, 0., "Home shows the start again");
}

fn page(content: code_block::Content<'static, ()>) -> Element<'static, (), Theme> {
    use iced::widget::space;
    use icedcn::widget::scrollable::scrollable;
    scrollable(column![code_block![content], space().height(2000)].width(iced::Fill))
        .width(iced::Fill)
        .height(iced::Fill)
        .into()
}

fn lines(count: usize) -> String {
    (1..=count).map(|n| format!("let x{n} = {n};\n")).collect()
}

#[test]
fn a_vertical_wheel_over_an_editor_that_shows_every_line_scrolls_the_page() {
    let mut ui = Simulator::with_size(
        Default::default(),
        Size::new(400., 300.),
        page(
            code_block::content("")
                .language("rust")
                .default_value(lines(5)),
        ),
    );
    let (line, _) = editor(&mut ui);
    ui.point_at(Point::new(line.x + 40., line.y + 20.));
    ui.scroll(wheel(0., -50.));
    let scrolled = offsets(&mut ui);
    assert_eq!(scrolled[0].2, Vector::new(0., 50.), "the page scrolls");
    assert_eq!(scrolled[1].2, Vector::ZERO);
}

#[test]
fn a_vertical_wheel_over_a_capped_editor_scrolls_the_editor_and_then_the_page() {
    let mut ui = Simulator::with_size(
        Default::default(),
        Size::new(400., 300.),
        page(
            code_block::content("")
                .language("rust")
                .max_height(120)
                .default_value(lines(20)),
        ),
    );
    let (line, _) = editor(&mut ui);
    ui.point_at(Point::new(line.x + 40., line.y + 20.));
    ui.scroll(wheel(0., -50.));
    let scrolled = offsets(&mut ui);
    assert_eq!(scrolled[0].2, Vector::ZERO, "the page stays");
    assert_eq!(scrolled[1].2, Vector::new(0., 50.), "the editor scrolls");
    ui.scroll(wheel(0., -10_000.));
    let (bounds, content, offset) = offsets(&mut ui)[1];
    assert_eq!(offset.y, content.height - bounds.height);
    ui.scroll(wheel(0., -50.));
    assert_eq!(
        offsets(&mut ui)[0].2,
        Vector::new(0., 50.),
        "the end bubbles"
    );
}
