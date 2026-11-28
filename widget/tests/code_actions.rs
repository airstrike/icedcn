use iced_core::Point;
use iced_test::simulator::{self, simulator};
use iced_widget::Space;
use icedcn_widget::{Inherited, View, button, code_block};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Wrap,
}

#[test]
fn plain_buttons_in_actions_publish_and_obey_ancestor_disabling() {
    for disabled in [false, true] {
        let wrap = button(Space::new().width(14).height(14))
            .pressed(true)
            .on_press(Message::Wrap);
        let mut slot: View<'_, Message> = code_block::actions(wrap).into();
        slot.inherit(&Inherited::default().with_disabled(disabled));
        let mut ui = simulator(slot.compose());
        ui.point_at(Point::new(10.0, 10.0));
        let _ = ui.simulate(simulator::click());
        let messages = ui.into_messages().collect::<Vec<_>>();
        assert_eq!(
            messages,
            if disabled {
                vec![]
            } else {
                vec![Message::Wrap]
            }
        );
    }
}
