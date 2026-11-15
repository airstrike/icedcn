use browser::component::field::FieldPage;
use iced::keyboard::key::Named;
use iced::{Element, Fill, Size};
use iced_test::Simulator;
use icedcn::Theme;
use icedcn::widget::{field, input, textarea};

#[test]
fn text_labels_focus_their_controls_and_only_the_active_control_receives_typing() {
    let contents: Element<'_, String, Theme> = field::group([
        field(
            input("First")
                .width(Fill)
                .on_input(|s| format!("input:{s}")),
        )
        .label("Name")
        .into(),
        field(textarea("Details").on_input(|s| format!("notes:{s}")))
            .label("Notes")
            .into(),
    ])
    .into();
    let mut ui = Simulator::new(contents);
    ui.click("Name").unwrap();
    ui.typewrite("A");
    ui.click("Notes").unwrap();
    ui.typewrite("B");
    assert_eq!(ui.drain().collect::<Vec<_>>(), ["input:A", "notes:B"]);
}

#[test]
fn registered_select_and_slider_accept_the_keyboard_after_clicking_their_labels() {
    let p = FieldPage::new(0);
    let mut ui = Simulator::new(p.preview("select"));
    ui.click("Department").unwrap();
    ui.tap_key(Named::Enter);
    ui.draw(&Theme::light());
    ui.find("Engineering").unwrap();
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    let mut ui = ui.rebuild(p.preview("select"));
    ui.draw(&Theme::light());
    ui.find("Design").unwrap();
    assert!(ui.find("Engineering").is_err());

    let mut ui = Simulator::new(p.preview("slider"));
    ui.click("Price Range").unwrap();
    ui.tap_key(Named::ArrowRight);
    ui.find("Set your budget range ($210 - 800).").unwrap();
}

#[test]
fn checkout_validates_required_fields_and_the_enter_key_submits_again() {
    let p = FieldPage::new(0);
    let mut ui = Simulator::with_size(Default::default(), Size::new(448., 850.), p.preview("demo"));
    ui.click("Submit").unwrap();
    for (name, value) in [
        ("Name on Card", "Ada"),
        ("Card Number", "1234567890123456"),
        ("CVV", "123"),
    ] {
        let error = format!("{name} is required.");
        ui.find(error.as_str()).unwrap();
        ui.click(name).unwrap();
        ui.typewrite(value);
        assert!(ui.find(error.as_str()).is_err());
    }
    ui.tap_key(Named::Enter);
    ui.find("Form submitted.").unwrap();
}

#[test]
fn profile_validates_a_blank_submission_at_phone_width() {
    let p = FieldPage::new(0);
    let mut ui = Simulator::with_size(
        Default::default(),
        Size::new(260., 650.),
        p.preview("responsive"),
    );
    ui.click("Submit").unwrap();
    ui.find("Name is required.").unwrap();
    ui.click("Name").unwrap();
    ui.typewrite("Ada");
    assert!(ui.find("Name is required.").is_err());
    ui.tap_key(Named::Enter);
    ui.find("Form submitted.").unwrap();
}

#[test]
fn switch_enters_the_tab_order_once_and_disabled_switches_stay_out() {
    use iced_test::core::widget::operation::{Operation, Outcome, focusable};
    use icedcn::widget::switch;
    let controls: Element<'_, bool, Theme> = field::group([
        field(switch().disabled(true)).label("Disabled").into(),
        field(switch().on_checked_change(|checked| checked))
            .label("Available")
            .into(),
    ])
    .into();
    let mut ui = Simulator::new(controls);
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut next) = next.finish() {
        ui.operate(next.as_mut());
    }
    ui.tap_key(Named::Space);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true, false]);
}

#[test]
fn choice_cards_activate_their_radio_from_the_description() {
    use icedcn::Horizontal;
    use icedcn::widget::radio_group;
    let choices: Element<'_, String, Theme> = radio_group(|group| {
        field::group([("a", "First"), ("b", "Second")].map(|(id, name)| {
            field(group.item(id))
                .orientation(Horizontal)
                .control_last()
                .label(name)
                .description(format!("Choose {name}"))
                .card()
                .into()
        }))
        .into()
    })
    .default_value("a")
    .on_value_change(|chosen| chosen)
    .into();
    let mut ui = Simulator::new(choices);
    ui.click("Choose Second").unwrap();
    ui.draw(&Theme::light());
    assert_eq!(ui.drain().collect::<Vec<_>>(), ["b"]);
}

#[test]
fn responsive_profile_stretches_fixed_controls_only_when_stacked() {
    use iced::Rectangle;
    use iced_test::core::widget::operation::Focusable;
    use iced_test::core::widget::{Id, Operation};
    #[derive(Default)]
    struct Widths(Vec<f32>);
    impl Operation for Widths {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn Focusable) {
            self.0.push(bounds.width);
        }
    }
    let page = FieldPage::new(0);
    for (panel, input_width) in [(292., 292.), (512., 192.)] {
        let mut ui = Simulator::with_size(
            Default::default(),
            Size::new(panel, 600.),
            page.preview("responsive"),
        );
        let mut controls = Widths::default();
        ui.operate(&mut controls);
        let [input, submit, cancel] = controls.0.as_slice() else {
            panic!("expected a text input and two actions")
        };
        assert_eq!(*input, input_width);
        if panel == 292. {
            assert_eq!((*submit, *cancel), (panel, panel));
        } else {
            assert!(*submit < 100. && *cancel < 100.);
        }
    }
}

#[test]
fn description_above_keeps_the_heading_compact_in_the_input_and_slider_recipes() {
    let page = FieldPage::new(0);
    for (id, title, detail) in [
        ("input", "Password", "Must be at least 8 characters long."),
        (
            "slider",
            "Price Range",
            "Set your budget range ($200 - 800).",
        ),
    ] {
        let mut ui = Simulator::new(page.preview(id));
        let label = ui.find(title).unwrap().bounds();
        let description = ui.find(detail).unwrap().bounds();
        assert_eq!(description.y - (label.y + label.height), 4., "{id}");
    }
}
