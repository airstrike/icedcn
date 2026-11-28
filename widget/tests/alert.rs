//! Alert actions keep the child's messages and respect ancestor state.

use iced_core::keyboard::key::Named;
use iced_core::widget::Operation;
use iced_core::widget::operation::{Outcome, focusable};
use iced_core::{Point, Size};
use iced_test::{Simulator, simulator};
use iced_widget::{Renderer, Space};
use icedcn_core::Theme;
use icedcn_widget::{Inherited, View, alert, button};

#[test]
fn actions_respect_inherited_disabled_state() {
    for disabled in [false, true] {
        let action = button(Space::new().width(24).height(16))
            .width(40)
            .height(24)
            .on_press(());
        // The null renderer does not shape a title; a fixed icon gives the
        // callout enough height to contain the complete action hitbox.
        let mut content: View<'_> = alert()
            .icon(Space::new().width(16).height(24))
            .title("Action")
            .action(action)
            .max_width(448)
            .into();
        content.inherit(&Inherited::default().with_disabled(disabled));
        let mut ui: Simulator<'_, (), Theme, Renderer> = Simulator::with_size(
            Default::default(),
            Size::new(448.0, 240.0),
            content.compose(),
        );

        ui.point_at(Point::new(420.0, 20.0));
        for _ in 0..2 {
            ui.simulate(simulator::click());
            assert_eq!(ui.drain().count(), usize::from(!disabled));
        }

        ui.point_at(Point::new(2.0, 2.0));
        ui.simulate(simulator::click());
        assert_eq!(ui.drain().count(), 0);

        let mut focus = focusable::focus_next();
        ui.operate(&mut focus);
        if let Outcome::Chain(mut operation) = focus.finish() {
            ui.operate(operation.as_mut());
        }
        for key in [Named::Enter, Named::Space] {
            ui.tap_key(key);
            assert_eq!(ui.drain().count(), usize::from(!disabled));
        }

        ui.point_at(Point::new(2.0, 2.0));
        ui.simulate(simulator::click());
        assert_eq!(ui.drain().count(), 0);
    }
}
