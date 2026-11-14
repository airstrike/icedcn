//! Focused geometry and consumer regressions for the installed Button page.
use browser::component::button::ButtonPage;
use browser::page::Page;
use browser::{Message, docs};
use iced::keyboard::key::Named;
use iced::{Point, Rectangle, Size};
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

#[test]
fn size_uses_window_breakpoint_and_retains_all_eight_controls() {
    let mut page = ButtonPage::new(0);
    for (window, width, horizontal) in [
        (1440., 558., true),
        (640., 558., true),
        (639., 324., false),
        (390., 260., false),
    ] {
        page.window_width(window);
        let mut ui = preview(&*page, "size", width);
        let bounds = controls(&mut ui);
        assert_eq!(bounds.len(), 8);
        for (index, pair) in bounds.chunks_exact(2).enumerate() {
            let h = 24. + 4. * index as f32;
            assert_eq!(pair[0].height, h);
            assert_eq!(pair[1].size(), Size::new(h, h));
            assert!((pair[1].x - pair[0].x - pair[0].width - 8.).abs() < 0.01);
            assert_eq!(pair[0].y, pair[1].y);
            assert!(pair[1].x + pair[1].width <= width);
            if index > 0 {
                assert_eq!(pair[0].y == bounds[0].y, horizontal);
                if !horizontal {
                    assert_eq!(pair[0].x, bounds[0].x);
                }
            }
        }
    }
}

#[test]
fn spinner_spacing_fits_narrow_frame_and_loading_controls_are_not_focusable() {
    let page = ButtonPage::new(0);
    for width in [558., 260.] {
        let mut ui = preview(&*page, "spinner", width);
        ui.draw(&Theme::light());
        let first = ui.find("Generating").expect("first loading label").bounds();
        let second = ui
            .find("Downloading")
            .expect("second loading label")
            .bounds();
        assert_eq!(first.y, second.y);
        // Remaining padding 11 + outer gap 8 + next leading padding 9.
        assert!((second.x - first.x - first.width - 28.).abs() < 0.01);
        assert!(second.x + second.width + 16. + 6. + 11. <= width);
        assert!(controls(&mut ui).is_empty());
        next(&mut ui);
        ui.tap_key(Named::Enter);
        ui.tap_key(Named::Space);
        assert_eq!(ui.drain().count(), 0);
    }
}

#[test]
fn anchor_and_handlerless_examples_preserve_the_public_action_contract() {
    let page = ButtonPage::new(0);
    let mut ui = preview(&*page, "link-button", 260.);
    ui.point_at(Point::new(12., 12.));
    ui.simulate(simulator::click());
    assert!(matches!(
        ui.drain().next(),
        Some(Message::OpenUrl("https://github.com/login"))
    ));
    ui.point_at(Point::new(220., 200.));
    ui.simulate(simulator::click());
    next(&mut ui);
    ui.tap_key(Named::Enter);
    assert!(matches!(
        ui.drain().next(),
        Some(Message::OpenUrl("https://github.com/login"))
    ));
    for id in [
        "demo",
        "default",
        "outline",
        "secondary",
        "ghost",
        "destructive",
        "link",
        "icon",
        "with-icon",
        "rounded",
    ] {
        let mut ui = preview(&*page, id, 260.);
        assert!(!controls(&mut ui).is_empty());
        ui.point_at(Point::new(12., 12.));
        for _ in 0..2 {
            ui.simulate(simulator::click());
        }
        assert_eq!(ui.drain().count(), 0);
        assert!(!controls(&mut ui).is_empty());
    }
}

#[test]
fn trailing_joined_controls_keep_their_label_and_icon_above_the_surface() {
    use iced::{Color, Renderer, mouse};
    use iced_test::core::Renderer as _;
    use iced_test::core::renderer::{Headless, Scale, Style};
    use iced_test::futures::futures::executor::block_on;
    use iced_test::runtime::UserInterface;

    let mut renderer = block_on(Renderer::new(Default::default(), Some("wgpu"))).expect("WGPU");
    renderer.hint(Scale {
        window: 1.,
        application: 1.,
    });
    let page = ButtonPage::new(0);
    let size = Size::new(360u32, 64);
    let cursor = mouse::Cursor::Unavailable;

    for dark in [false, true] {
        let theme = docs::theme(dark);
        let mut ui = UserInterface::build(
            page.preview("button-group"),
            Size::new(360., 64.),
            Default::default(),
            &mut renderer,
        );
        let mut controls = Controls::default();
        ui.operate(&renderer, &mut controls);
        ui.draw(
            &mut renderer,
            &theme,
            &Style {
                text_color: theme.neutral.foreground.into(),
            },
            cursor,
        );
        let pixels = renderer.screenshot(size, 1., Color::from(theme.neutral.base));
        assert_eq!(controls.0.len(), 5);
        for (index, name) in [(2, "Report"), (4, "ellipsis")] {
            let bounds = controls.0[index].shrink(6.);
            let ink = (bounds.y.ceil() as u32..(bounds.y + bounds.height) as u32)
                .flat_map(|y| {
                    (bounds.x.ceil() as u32..(bounds.x + bounds.width) as u32)
                        .map(move |x| ((y * size.width + x) * 4) as usize)
                })
                .filter(|&i| {
                    if dark {
                        pixels[i] > 170
                    } else {
                        pixels[i] < 90
                    }
                })
                .count();
            assert!(
                ink >= 5,
                "{name} disappeared in dark={dark}, with {ink} ink pixels"
            );
        }
    }
}
