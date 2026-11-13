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
