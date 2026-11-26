//! Layout of the collapsed Select and its native options.
#![cfg(any(feature = "wgpu", feature = "vello-cpu"))]

use iced_core::keyboard::key::Named;
use iced_core::widget::Id;
use iced_core::{Element, Pixels, Rectangle};
use iced_test::simulator::simulator;
use iced_widget::{Renderer, Widget, text};
use icedcn_core::Theme;
use icedcn_widget::{Size, container, select};

const LABEL: &str = "Quarterly revenue";

fn label_bounds(size: f32, line: f32) -> Rectangle {
    let label: Element<'_, (), Theme, Renderer> =
        text(LABEL).size(size).line_height(Pixels(line)).boxed();
    simulator(label).find(LABEL).unwrap().bounds()
}

#[test]
fn trigger_and_label_use_the_requested_metrics() {
    for (preset, height, typography, expected_height) in [
        (Size::Default, None, None, 32.),
        (Size::Sm, None, None, 28.),
        (Size::Lg, None, None, 32.),
        (Size::Default, Some(24.), Some((12., 16.)), 24.),
        (Size::Sm, Some(36.), Some((13., 19.5)), 36.),
    ] {
        for selected in [false, true] {
            let mut control = select([select::item("revenue", LABEL)]).placeholder(LABEL);
            if let Some(height) = height {
                control = control.height(height);
            }
            if let Some((size, line)) = typography {
                control = control.text_size(size).line_height(Pixels(line));
            }
            if selected {
                control = control.value("revenue");
            }
            let element: Element<'_, (), Theme, Renderer> =
                container(control.size(preset)).id("trigger").into();
            let mut ui = simulator(element);
            let trigger = ui.find(Id::new("trigger")).unwrap().bounds();
            let actual = ui.find(LABEL).unwrap().bounds();
            let (size, line) = typography.unwrap_or((14., 20.));
            let expected = label_bounds(size, line);
            assert_eq!(trigger.height, expected_height);
            assert_eq!(actual.size(), expected.size(), "selected={selected}");
            assert!(actual.width > 0.);
            assert!((actual.center_y() - trigger.center_y()).abs() < 1.);
        }
    }
}

#[test]
fn resized_trigger_keeps_native_option_typography_and_keyboard_selection() {
    let control = select([select::item("revenue", LABEL)])
        .placeholder("Choose a measure")
        .height(24.)
        .text_size(12.)
        .line_height(Pixels(16.))
        .on_value_change(|value| value);
    let element: Element<'_, String, Theme, Renderer> = control.into();
    let mut ui = simulator(element);
    ui.click("Choose a measure").unwrap();
    ui.draw(&Theme::light());
    let option = ui.find(LABEL).unwrap().bounds();
    assert_eq!(option.size(), label_bounds(14., 20.).size());
    ui.tap_key(Named::Enter);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), ["revenue"]);
}
