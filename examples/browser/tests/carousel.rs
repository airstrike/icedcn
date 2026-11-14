//! Carousel's source frame overrides, slide geometry and API caption.
use std::borrow::Cow;
use std::time::Duration;

use browser::component::carousel::{self, CarouselPage};
use browser::{Message, font};
use iced::time::Instant;
use iced::{Event, Font, Rectangle, Size, window};
use iced_test::core::Settings;
use iced_test::core::widget::operation::Focusable;
use iced_test::core::widget::{Id, Operation};
use iced_test::simulator::{self, Simulator};
use icedcn::Theme;

#[derive(Default, Debug)]
struct Bounds {
    containers: Vec<Rectangle>,
    controls: Vec<Rectangle>,
}

impl Operation for Bounds {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn container(&mut self, _: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
        self.containers.push(bounds);
    }
    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn Focusable) {
        self.controls.push(bounds);
    }
}

fn bounds(ui: &mut Simulator<'_, Message, Theme, iced::Renderer>) -> Bounds {
    let mut bounds = Bounds::default();
    ui.operate(&mut bounds);
    bounds
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.1
}
fn has_card(bounds: &Bounds, width: f32, height: f32) -> bool {
    bounds
        .containers
        .iter()
        .any(|b| close(b.width, width) && close(b.height, height))
}
fn settings() -> Settings {
    Settings {
        // Use the bundled font so line-box geometry does not depend on host fonts.
        // The monitor checks the application's Geist appearance separately.
        font: Font::new(font::GEIST_MONO_NAME),
        fonts: vec![Cow::Borrowed(font::GEIST_MONO)],
        ..Settings::default()
    }
}

#[test]
fn source_card_geometry_controls_and_frame_overrides_at_two_widths() {
    let theme = Theme::light();
    for (window_width, panel_width) in [(1440.0, 638.0), (390.0, 340.0)] {
        let mut page = CarouselPage::new(0);
        page.window_width(window_width);
        let desktop = window_width > 640.0;
        for (id, card_width, card_height, frame_height) in if desktop {
            [
                ("demo", 312.0, 344.0, 512.0),
                ("size", 109.333, 141.333, 288.0),
                ("spacing", 117.333, 149.333, 288.0),
                ("orientation", 312.0, 116.0, 512.0),
                ("api", 318.0, 350.0, 512.0),
                ("plugin", 312.0, 344.0, 512.0),
            ]
        } else {
            [
                ("demo", 184.0, 216.0, 320.0),
                ("size", 80.0, 116.0, 288.0),
                ("spacing", 86.0, 118.0, 288.0),
                ("orientation", 252.0, 116.0, 512.0),
                ("api", 158.0, 190.0, 288.0),
                ("plugin", 152.0, 184.0, 288.0),
            ]
        } {
            let mut ui = Simulator::with_size(
                settings(),
                Size::new(panel_width, 600.0),
                page.docs_preview(id, &theme),
            );
            let observed = bounds(&mut ui);
            assert!(
                has_card(&observed, card_width, card_height),
                "window={window_width} {id}: missing {card_width}x{card_height}; {observed:?}"
            );
            assert!(
                observed
                    .containers
                    .iter()
                    .any(|b| close(b.width, panel_width) && close(b.height, frame_height)),
                "{id}: frame height"
            );
            assert_eq!(
                observed.controls.len(),
                1,
                "previous is disabled on mount: {id}"
            );
            for control in observed.controls {
                assert!(
                    close(control.width, 28.0) && close(control.height, 28.0),
                    "{id}: {control:?}"
                );
                assert!(
                    control.x >= 0.0
                        && control.y >= 0.0
                        && control.x + control.width <= panel_width
                        && control.y + control.height <= frame_height,
                    "clipped control: {id} {control:?}"
                );
            }
            let label = ui.find("1").expect("first slide is mounted").bounds();
            let line_height = match id {
                "size" | "orientation" => 36.0,
                "spacing" => 32.0,
                _ => 40.0,
            };
            assert!(close(label.height, line_height), "{id}: {label:?}");
            assert!(
                label.x >= 0.0
                    && label.y >= 0.0
                    && label.x + label.width <= panel_width
                    && label.y + label.height <= frame_height,
                "label clipping {id}: {label:?}"
            );
            if id == "api" {
                assert!(ui.find("Slide 1 of 5").is_ok());
            }
            assert!(
                ui.find("5")
                    .expect("last slide remains mounted")
                    .visible_bounds()
                    .is_none(),
                "{id}: off-track content must not expose visible bounds"
            );
        }
    }
}

#[test]
fn actual_window_breakpoints_do_not_depend_on_embed_width() {
    for (window_width, expected_width, expected_height) in [
        (639.0, 80.0, 116.0),
        (640.0, 144.0, 176.0),
        (767.0, 144.0, 176.0),
        (768.0, 176.0, 208.0),
        (1023.0, 176.0, 208.0),
        (1024.0, 109.333, 141.333),
    ] {
        let mut ui = Simulator::with_size(
            settings(),
            Size::new(600.0, 600.0),
            carousel::size(window_width).compose(),
        );
        assert!(
            has_card(&bounds(&mut ui), expected_width, expected_height),
            "window={window_width}"
        );
    }
    // A narrow embed in a desktop window still divides into three cells.
    let mut ui = Simulator::with_size(
        settings(),
        Size::new(340.0, 600.0),
        carousel::size(1440.0).compose(),
    );
    // 340 - 96 controls = 244px track; (244+16)/3 -16 -8 = 62.667px card.
    assert!(has_card(&bounds(&mut ui), 62.667, 116.0));
}

#[test]
fn api_count_follows_the_installed_controlled_component() {
    let page = CarouselPage::new(0);
    let theme = Theme::light();
    let mut ui = Simulator::with_size(
        settings(),
        Size::new(638.0, 600.0),
        page.docs_preview("api", &theme),
    );
    assert!(ui.find("Slide 1 of 5").is_ok());
    let next = bounds(&mut ui).controls[0];
    ui.point_at(next.center());
    ui.simulate(simulator::click());
    assert!(ui.find("Slide 2 of 5").is_ok());
    assert!(
        ui.drain().next().is_none(),
        "selection belongs to the example Component"
    );
    let now = Instant::now();
    for n in 0..180 {
        ui.simulate([Event::Window(window::Event::RedrawRequested(
            now + Duration::from_millis(n * 17),
        ))]);
    }
    let text = ui.find("2").expect("second slide").bounds();
    assert!(
        text.center_x() > 300.0 && text.center_x() < 340.0,
        "{text:?}"
    );
}
