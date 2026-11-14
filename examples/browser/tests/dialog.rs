//! Geometry and interaction of the installed Dialog recipes.
use browser::Message;
use browser::component::dialog::DialogPage;
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
        Instant::now() + Duration::from_millis(160),
    ))]);
    ui = ui.rebuild(page.preview(id));
    ui.draw(theme);
    ui
}

#[test]
fn profile_focus_save_and_responsive_footer_match_the_recipe() {
    let page = DialogPage::new(0);
    for width in [390., 1440.] {
        let theme = Theme::light();
        let mut ui = Ui::with_size(
            Default::default(),
            Size::new(width, 900.),
            page.preview("demo"),
        );
        redraw(&mut ui, &theme);
        ui.click("Open Dialog").unwrap();
        redraw(&mut ui, &theme);
        let fields = contents(&mut ui).inputs;
        assert_eq!(fields[0].0, "Pedro Duarte");
        assert_eq!(fields[0].1.width, (width - 32.).min(384.) - 32.);
        assert_eq!(fields[1].0, "@peduarte");
        let focused = ui.find(selector::is_focused()).unwrap().bounds();
        assert_eq!(focused, fields[0].1);
        ui.typewrite("Q");
        assert_eq!(contents(&mut ui).inputs[0].0, "Q");
        let cancel = ui.find("Cancel").unwrap().bounds();
        let save = ui.find("Save changes").unwrap().bounds();
        assert_eq!(cancel.y > save.y, width < 640.);
        ui.tap_key(Named::Tab);
        assert_eq!(
            ui.find(selector::is_focused()).unwrap().bounds(),
            fields[1].1
        );
        ui.tap_key(Named::Tab);
        let cancel_button = ui.find(selector::is_focused()).unwrap().bounds();
        assert!(cancel_button.contains(cancel.center()));
        if width < 640. {
            assert_eq!(cancel_button.width, width - 64.);
        }
        ui.click("Save changes").unwrap();
        ui.find("Edit profile").unwrap();
        ui.click("Cancel").unwrap();
        ui = finish_exit(ui, &*page, "demo", &theme);
        assert!(ui.find("Edit profile").is_err());
        assert!(
            ui.find(selector::is_focused())
                .unwrap()
                .bounds()
                .contains(ui.find("Open Dialog").unwrap().bounds().center())
        );
        ui.tap_key(Named::Enter);
        redraw(&mut ui, &theme);
        assert_eq!(contents(&mut ui).inputs[0].0, "Pedro Duarte");
    }
}

#[test]
fn share_link_is_read_only_and_close_aligns_with_the_field() {
    let page = DialogPage::new(0);
    let theme = Theme::dark();
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(1440., 900.),
        page.preview("custom-close-button"),
    );
    redraw(&mut ui, &theme);
    ui.click("Share").unwrap();
    redraw(&mut ui, &theme);
    let before = contents(&mut ui).inputs;
    assert_eq!(before[0].1.width, 416.);
    ui.typewrite("X");
    assert_eq!(contents(&mut ui).inputs, before);
    ui.tap_key(Named::Tab);
    assert_eq!(
        ui.find(selector::is_focused()).unwrap().bounds().x,
        before[0].1.x
    );
    ui.click("Close").unwrap();
    ui = finish_exit(ui, &*page, "custom-close-button", &theme);
    assert!(ui.find("Share link").is_err());
}

#[test]
fn scrolling_a_half_window_body_keeps_the_header_and_footer_fixed() {
    let page = DialogPage::new(0);
    for (id, title) in [
        ("sticky-footer", "Sticky Footer"),
        ("scrollable-content", "Scrollable Content"),
    ] {
        for (width, height) in [(1440., 900.), (390., 844.)] {
            let theme = Theme::light();
            let mut ui = Ui::with_size(
                Default::default(),
                Size::new(width, height),
                page.preview(id),
            );
            redraw(&mut ui, &theme);
            ui.click(title).unwrap();
            redraw(&mut ui, &theme);
            let body = contents(&mut ui).scrolling.unwrap();
            assert_eq!(body.0.height, height * 0.5);
            let header = if id == "sticky-footer" {
                "This dialog has a sticky footer that stays visible while the content scrolls."
            } else {
                "This is a dialog with scrollable content."
            };
            let head = ui.find(header).unwrap().bounds();
            let close = ui.find("Close").ok().map(|e| e.bounds());
            ui.point_at(body.0.center());
            ui.scroll(mouse::ScrollDelta::Pixels { x: 0., y: -140. });
            redraw(&mut ui, &theme);
            let after = contents(&mut ui).scrolling.unwrap();
            assert_ne!(body.1, after.1);
            assert_eq!(body.0, after.0);
            assert_eq!(ui.find(header).unwrap().bounds(), head);
            assert_eq!(ui.find("Close").ok().map(|e| e.bounds()), close);
            ui.tap_key(Named::Escape);
            ui = finish_exit(ui, &*page, id, &theme);
            assert!(ui.find(header).is_err());
        }
    }
}

#[test]
fn the_panel_without_close_controls_dismisses_from_the_backdrop() {
    use iced_test::simulator;
    let page = DialogPage::new(0);
    let theme = Theme::light();
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(390., 844.),
        page.preview("no-close-button"),
    );
    redraw(&mut ui, &theme);
    ui.click("No Close Button").unwrap();
    redraw(&mut ui, &theme);
    let description = "This dialog doesn't have a close button in the top-right corner.";
    ui.find(description).unwrap();
    assert!(ui.find(selector::is_focused()).is_err());
    ui.point_at([4., 4.]);
    ui.simulate(simulator::click());
    ui = finish_exit(ui, &*page, "no-close-button", &theme);
    assert!(ui.find(description).is_err());
}
