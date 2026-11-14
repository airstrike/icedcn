//! Regression coverage for the Input page's form layout and editing examples.
use browser::component::input::InputPage;
use browser::{Message, docs, page};
use iced::keyboard::key::Named;
use iced::{Rectangle, Size};
use iced_test::core::Settings;
use iced_test::core::widget::Id;
use iced_test::core::widget::operation::{Operation, TextInput};
use iced_test::{Simulator, simulator};

#[derive(Default)]
struct Bounds(Vec<Rectangle>);

impl Operation for Bounds {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn TextInput) {
        self.0.push(bounds);
    }
}

#[test]
fn form_and_grid_controls_fill_their_documented_columns() {
    let mut page = InputPage::new(0);
    for (window, frame, group, grid) in [(1440., 638., 384_f32, 384.), (390., 340., 260., 292.)] {
        page.window_width(window);
        for (id, widths) in [
            ("form", vec![group, group, (group - 16.) / 2., group]),
            ("fieldgroup", vec![group.min(320.); 2]),
            ("grid", vec![(grid - 20.) / 2.; 2]),
        ] {
            let theme = docs::theme(false);
            let mut ui = Simulator::with_size(
                Settings::default(),
                Size::new(frame, 512.),
                page.docs_preview(id, &theme),
            );
            let mut inputs = Bounds::default();
            ui.operate(&mut inputs);
            let widths_found: Vec<_> = inputs.0.iter().map(|r| r.width).collect();
            assert_eq!(widths_found, widths, "{id} at {window}");
            assert!(inputs.0.iter().all(|r| r.height == 32.));
        }
    }
}

#[test]
fn form_enter_validates_and_group_reset_clears_both_inputs() {
    let page = InputPage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(400., 512.),
        page.preview("form"),
    );
    ui.click("Name").unwrap();
    ui.tap_key(Named::Enter);
    ui.find("Please fill out this field.").unwrap();
    ui.typewrite("Jordan");
    ui.click("Email").unwrap();
    ui.typewrite("missing-domain");
    ui.tap_key(Named::Enter);
    ui.find("Enter an email address.").unwrap();
    ui.click("Cancel").unwrap();
    assert!(ui.find("missing-domain").is_err());

    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(400., 288.),
        page.preview("fieldgroup"),
    );
    ui.click("Name").unwrap();
    ui.typewrite("Jordan");
    ui.click("Email").unwrap();
    ui.typewrite("team@example.org");
    ui.click("Reset").unwrap();
    assert!(ui.find("Jordan").is_err());
    assert!(ui.find("team@example.org").is_err());
}

#[test]
fn file_selection_displays_the_chosen_name_and_keeps_it_on_cancel() {
    let mut page = InputPage::new(7);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(400., 288.),
        page.preview("file"),
    );
    ui.click("Choose File").unwrap();
    assert!(
        ui.drain()
            .any(|m| matches!(m, Message::Page(7, page::Message::ChooseFile)))
    );
    drop(ui);
    let _ = page.update(page::Message::FileSelected(Some("picture.jpg".into())));
    let _ = page.update(page::Message::FileSelected(None));
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(400., 288.),
        page.preview("file"),
    );
    ui.find("picture.jpg").unwrap();
}

#[test]
fn a_field_label_focuses_its_input_and_a_disabled_field_rejects_edits() {
    let page = InputPage::new(0);
    let mut field = Simulator::with_size(
        Settings::default(),
        Size::new(320., 288.),
        page.preview("field"),
    );
    field.click("Username").unwrap();
    field.typewrite("jordan");
    field.find("jordan").unwrap();

    let mut disabled = Simulator::with_size(
        Settings::default(),
        Size::new(320., 288.),
        page.preview("disabled"),
    );
    disabled.point_at(iced::Point::new(32., 43.));
    disabled.simulate(simulator::click());
    disabled.typewrite("unavailable");
    assert!(disabled.find("unavailable").is_err());
}
