//! Keyboard selection survives a view rebuild; a controlled carousel only
//! publishes a new snap. Autoplay runs on its own scheduled redraw.
use iced_core::keyboard::key::Named;
use iced_core::time::Instant;
use iced_core::{Element, Event, Point, window};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::{carousel, container};
use std::time::Duration;

fn gallery(controlled: bool) -> Element<'static, usize, Theme, Renderer> {
    let mut c = carousel(5, |_| Space::new().width(100).height(100))
        .on_value_change(|i| i)
        .autoplay(Duration::from_secs(2));
    if controlled {
        c = c.value(0);
    }
    container(c).width(220).into()
}
#[test]
fn keyboard_clamps_and_respects_controlled_snap() {
    for (controlled, expected) in [(true, vec![1, 1]), (false, vec![1, 2])] {
        let mut ui = simulator(gallery(controlled));
        ui.point_at(Point::new(76.0, 40.0));
        let _ = ui.simulate(simulator::click());
        let _ = ui.simulate([simulator::press_key(Named::ArrowRight, None)]);
        let mut ui = ui.rebuild(gallery(controlled));
        let _ = ui.simulate([simulator::press_key(Named::ArrowRight, None)]);
        assert_eq!(ui.into_messages().collect::<Vec<_>>(), expected);
    }
}
#[test]
fn autoplay_uses_scheduled_redraw_and_pauses_on_hover() {
    let mut ui = simulator(gallery(false));
    ui.point_at(Point::new(300.0, 300.0));
    let now = Instant::now();
    let tick = |sec| {
        Event::Window(window::Event::RedrawRequested(
            now + Duration::from_secs(sec),
        ))
    };
    let _ = ui.simulate([tick(0), tick(2)]);
    ui.point_at(Point::new(60.0, 20.0));
    let _ = ui.simulate([tick(4), tick(6)]);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [1]);
}
