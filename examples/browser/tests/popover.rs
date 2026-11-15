//! Editable Popover recipes keep their label layout and keyboard focus inside the panel.
use browser::Message;
use browser::component::popover::PopoverPage;
use iced::keyboard::{self, Modifiers, key::Named};
use iced::time::Instant;
use iced::{Event, Rectangle, Size, window};
use iced_test::{Simulator, selector, simulator};
use iced_widget::core::widget::{Id, Operation, operation};
use icedcn::Theme;

#[derive(Default)]
struct Inputs(Vec<(String, Rectangle)>);

impl Operation for Inputs {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn text_input(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        input: &mut dyn operation::TextInput,
    ) {
        self.0.push((input.text().into(), bounds));
    }
}

type Ui<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn inputs(ui: &mut Ui<'_>) -> Vec<(String, Rectangle)> {
    let mut values = Inputs::default();
    ui.operate(&mut values);
    values.0
}

fn draw(ui: &mut Ui<'_>) {
    ui.draw(&Theme::light());
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    ui.draw(&Theme::light());
}

#[test]
fn dimension_labels_precede_wide_inputs_at_both_breakpoints() {
    for width in [390., 1440.] {
        let mut page = PopoverPage::new(0);
        page.window_width(width);
        for (id, trigger, expected, labels) in [
            (
                "demo",
                "Open popover",
                195.,
                &["Width", "Max. width", "Height", "Max. height"][..],
            ),
            ("form", "Open Popover", 155., &["Width", "Height"][..]),
        ] {
            let mut ui =
                Ui::with_size(Default::default(), Size::new(width, 844.), page.preview(id));
            draw(&mut ui);
            ui.click(trigger).unwrap();
            draw(&mut ui);
            let fields = inputs(&mut ui);
            assert_eq!(fields.len(), labels.len());
            for (label, (_, bounds)) in labels.iter().zip(fields) {
                let label = ui.find(*label).unwrap().bounds();
                assert!(label.x + label.width <= bounds.x);
                assert!(
                    (bounds.width - expected).abs() < 5.,
                    "{id} at {width}: {bounds:?}"
                );
                assert_eq!(bounds.height, 32.);
            }
        }
    }
}

#[test]
fn editable_panel_selects_text_traverses_controls_and_remounts_defaults() {
    let page = PopoverPage::new(0);
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(390., 844.),
        page.preview("form"),
    );
    draw(&mut ui);
    ui.click("Open Popover").unwrap();
    draw(&mut ui);
    ui.typewrite("172px");
    assert_eq!(inputs(&mut ui)[0].0, "172px");
    ui.tap_key(Named::Tab);
    ui.typewrite("64px");
    assert_eq!(inputs(&mut ui)[1].0, "64px");
    ui.tap_key(Named::Tab);
    ui.typewrite("200px");
    assert_eq!(inputs(&mut ui)[0].0, "200px");
    let mut reverse = simulator::press_key(Named::Tab, None);
    if let Event::Keyboard(keyboard::Event::KeyPressed { modifiers, .. }) = &mut reverse {
        *modifiers = Modifiers::SHIFT;
    }
    ui.simulate([reverse]);
    ui.typewrite("80px");
    assert_eq!(inputs(&mut ui)[1].0, "80px");
    ui.click("Width").unwrap();
    ui.typewrite("Q");
    assert!(inputs(&mut ui)[0].0.contains('Q'));
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(page.preview("form"));
    draw(&mut ui);
    assert!(inputs(&mut ui).is_empty());
    let trigger = ui.find("Open Popover").unwrap().bounds();
    assert!(
        ui.find(selector::is_focused())
            .unwrap()
            .bounds()
            .contains(trigger.center())
    );
    ui.tap_key(Named::Enter);
    draw(&mut ui);
    assert_eq!(inputs(&mut ui)[0].0, "100%");
    assert_eq!(inputs(&mut ui)[1].0, "25px");
}

#[test]
fn read_only_panel_owns_activation_keys_until_escape() {
    let page = PopoverPage::new(0);
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(390., 844.),
        page.preview("basic"),
    );
    draw(&mut ui);
    ui.click("Open Popover").unwrap();
    draw(&mut ui);
    for key in [Named::Tab, Named::Enter, Named::Space] {
        ui.tap_key(key);
        ui = ui.rebuild(page.preview("basic"));
        draw(&mut ui);
        ui.find("Dimensions").unwrap();
    }
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(page.preview("basic"));
    draw(&mut ui);
    assert!(ui.find("Dimensions").is_err());
}
