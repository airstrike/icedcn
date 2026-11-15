//! Geometry and interaction of the installed Sheet recipes.
use browser::Message;
use browser::component::sheet::SheetPage;
use iced::keyboard::key::Named;
use iced::time::{Duration, Instant};
use iced::{Event, Rectangle, Size, Vector, mouse, window};
use iced_test::{Simulator, selector};
use iced_widget::core::widget::{Id, Operation, operation};
use icedcn::Theme;

#[derive(Default)]
struct Contents {
    inputs: Vec<(String, Rectangle)>,
    scrolling: Option<(Rectangle, Vector)>,
}

impl Operation for Contents {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn text_input(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        input: &mut dyn operation::TextInput,
    ) {
        self.inputs.push((input.text().to_string(), bounds));
    }

    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        _: Size,
        offset: Vector,
        _: &mut dyn operation::Scrollable,
    ) {
        self.scrolling = Some((bounds, offset));
    }
}

type Ui<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn contents(ui: &mut Ui<'_>) -> Contents {
    let mut contents = Contents::default();
    ui.operate(&mut contents);
    contents
}

fn redraw(ui: &mut Ui<'_>, theme: &Theme) {
    ui.draw(theme);
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    ui.draw(theme);
}

fn finish_exit<'a>(
    mut ui: Ui<'a>,
    page: &'a dyn browser::page::Page,
    id: &str,
    theme: &Theme,
) -> Ui<'a> {
    ui = ui.rebuild(page.preview(id));
    ui.simulate([Event::Window(window::Event::RedrawRequested(
        Instant::now() + Duration::from_millis(260),
    ))]);
    ui = ui.rebuild(page.preview(id));
    ui.draw(theme);
    ui
}

#[test]
fn profile_fills_its_edge_focuses_name_and_keeps_edits_until_closed() {
    let theme = Theme::light();
    let mut page = SheetPage::new(0);
    for (width, height, panel) in [(1440., 900., 384.), (390., 844., 292.5), (600., 800., 450.)] {
        page.window_width(width);
        let mut ui = Ui::with_size(
            Default::default(),
            Size::new(width, height),
            page.preview("demo"),
        );
        redraw(&mut ui, &theme);
        ui.click("Open").unwrap();
        redraw(&mut ui, &theme);
        let fields = contents(&mut ui).inputs;
        assert_eq!(fields.len(), 2);
        let (name, bounds) = &fields[0];
        assert_eq!(name, "Pedro Duarte");
        assert!(
            (bounds.width - (panel - 33.)).abs() <= 1.5,
            "{width} {bounds:?}"
        );
        assert!((bounds.x - (width - panel + 17.)).abs() <= 1.5);
        assert!(bounds.y >= 100. && bounds.y < 155., "{bounds:?}");
        let label = ui.find("Name").unwrap().bounds();
        assert_eq!(bounds.y - (label.y + label.height), 12.);
        assert_eq!(ui.find(selector::is_focused()).unwrap().bounds(), *bounds);
        ui.typewrite("Ada");
        ui.tap_key(Named::Tab);
        assert_eq!(
            ui.find(selector::is_focused()).unwrap().bounds(),
            fields[1].1
        );
        ui.click("Name").unwrap();
        assert_eq!(ui.find(selector::is_focused()).unwrap().bounds(), *bounds);
        ui.click("Save changes").unwrap();
        assert_eq!(contents(&mut ui).inputs[0].0, "Ada");
        let close = ui.find("Close").unwrap().bounds();
        assert!(close.y > height - 58., "{height} {close:?}");
        ui.click("Close").unwrap();
        ui = finish_exit(ui, &*page, "demo", &theme);
        assert!(contents(&mut ui).inputs.is_empty());
        ui.click("Open").unwrap();
        redraw(&mut ui, &theme);
        assert_eq!(contents(&mut ui).inputs[0].0, "Pedro Duarte");
    }
}

#[test]
fn all_four_edges_scroll_the_body_and_keep_their_footer_on_screen() {
    let theme = Theme::dark();
    let page = SheetPage::new(0);
    for (width, height) in [(1440., 900.), (390., 844.)] {
        for edge in ["Top", "Right", "Bottom", "Left"] {
            let mut ui = Ui::with_size(
                Default::default(),
                Size::new(width, height),
                page.preview("side"),
            );
            redraw(&mut ui, &theme);
            ui.click(edge).unwrap();
            redraw(&mut ui, &theme);
            let (body, offset) = contents(&mut ui).scrolling.unwrap();
            let header = ui.find("Edit profile").unwrap().bounds();
            let footer = ui.find("Cancel").unwrap().bounds();
            let bottom = if edge == "Top" { height / 2. } else { height };
            assert!(
                footer.y > bottom - 60. && footer.y < bottom - 12.,
                "{edge} {footer:?}"
            );
            let top = if edge == "Bottom" { height / 2. } else { 0. };
            assert!((header.y - top - 16.).abs() < 6., "{edge} {header:?}");
            ui.point_at(body.center());
            ui.scroll(mouse::ScrollDelta::Pixels { x: 0., y: -180. });
            redraw(&mut ui, &theme);
            assert_ne!(
                contents(&mut ui).scrolling.unwrap().1,
                offset,
                "{edge} {body:?}"
            );
            assert_eq!(ui.find("Cancel").unwrap().bounds(), footer);
            assert_eq!(ui.find("Edit profile").unwrap().bounds(), header);
            ui.click("Cancel").unwrap();
            ui = finish_exit(ui, &*page, "side", &theme);
            assert!(ui.find("Edit profile").is_err());
        }
    }
}

#[test]
fn no_close_button_is_a_header_only_panel_that_dismisses_from_outside() {
    use iced_test::simulator;
    let page = SheetPage::new(0);
    let theme = Theme::light();
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(390., 844.),
        page.preview("no-close-button"),
    );
    redraw(&mut ui, &theme);
    ui.click("Open Sheet").unwrap();
    redraw(&mut ui, &theme);
    ui.find("No Close Button").unwrap();
    assert!(ui.find("Close").is_err() && contents(&mut ui).inputs.is_empty());
    ui.point_at((2., 400.));
    ui.simulate(simulator::click());
    ui = finish_exit(ui, &*page, "no-close-button", &theme);
    assert!(ui.find("No Close Button").is_err());
}
