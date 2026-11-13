use iced_core::time::Instant;
use iced_core::{Element, Event, Point, window};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedxdy_core::{Horizontal, Theme};
use icedxdy_widget::{Slot, field, switch};

#[test]
fn a_trailing_control_obeys_its_label_and_field_disabled_state() {
    for disabled in [false, true] {
        let label = Slot::from(Space::new().width(80).height(20));
        let content = field(switch().on_checked_change(|checked| checked))
            .orientation(Horizontal)
            .control_last()
            .label(label)
            .disabled(disabled)
            .width(160);
        let content: Element<'_, bool, Theme, Renderer> = content.into();
        let mut ui: iced_test::Simulator<'_, bool, Theme, Renderer> = simulator(content);
        ui.point_at(Point::new(12., 10.));
        let _ = ui.simulate(simulator::click());
        let _ = ui.simulate([Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )]);
        assert_eq!(
            ui.into_messages().collect::<Vec<_>>(),
            if disabled { vec![] } else { vec![true] }
        );
    }
}

#[test]
fn resizing_a_responsive_field_preserves_the_typed_value() {
    use iced_core::Size;
    use icedxdy_widget::{input, operation::control};
    let label: Slot<'_, String> = Space::new().width(40).height(20).into();
    let form: Element<'_, String, Theme, Renderer> = field(input("").on_input(|value| value))
        .label(label)
        .responsive()
        .into();
    let mut ui = simulator(form).resize(Size::new(540., 200.));
    ui.operate(&mut control::activate());
    let _ = ui.typewrite("abc");
    let mut ui = ui.resize(Size::new(300., 200.));
    let _ = ui.typewrite("d");
    assert_eq!(ui.into_messages().last().as_deref(), Some("abcd"));
}

#[test]
fn an_addon_focuses_its_input_except_when_a_button_handles_the_press() {
    use icedxdy_widget::{button, input, input_group};
    for active_button in [false, true] {
        let block: Slot<'_, String> = Space::new().width(22).height(20).into();
        let addon = if active_button {
            Slot::from(button(block).on_press("button".to_owned()))
        } else {
            block
        };
        let group: Element<'_, String, Theme, Renderer> =
            input_group(input("").on_input(|value| value))
                .addon(input_group::addon(addon))
                .into();
        let mut ui = simulator(group);
        ui.point_at(Point::new(16., 15.));
        let _ = ui.simulate(simulator::click());
        let _ = ui.typewrite("x");
        assert_eq!(
            ui.into_messages().collect::<Vec<_>>(),
            if active_button {
                vec!["button"]
            } else {
                vec!["x"]
            }
        );
    }
}

#[test]
fn one_time_code_filters_input_and_edits_the_clicked_slot() {
    use iced_core::keyboard::key::Named;
    use icedxdy_widget::input_otp::{Pattern, input_otp};
    use icedxdy_widget::operation::control;
    let code: Element<'_, String, Theme, Renderer> = input_otp(4)
        .pattern(Pattern::Digits)
        .group_size(2)
        .on_value_change(|code| code)
        .into();
    let mut ui = simulator(code);
    ui.operate(&mut control::activate());
    let _ = ui.typewrite("1a2345");
    ui.point_at(Point::new(90., 16.));
    let _ = ui.simulate(simulator::click());
    let _ = ui.simulate([simulator::press_key(Named::Backspace, None)]);
    let _ = ui.typewrite("8");
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        ["1", "12", "123", "1234", "134", "1834"]
    );
}
