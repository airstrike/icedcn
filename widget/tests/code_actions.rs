use iced_core::Point;
use iced_test::simulator::{self, simulator};
use iced_widget::Space;
use icedcn_widget::{Inherited, Slot, code_block};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Wrap,
}

#[test]
fn code_actions_publish_and_obey_ancestor_disabling() {
    for disabled in [false, true] {
        let action = code_block::action(Space::new().width(14).height(14), "Wrap lines")
            .active(true)
            .on_press(Message::Wrap);
        let mut slot: Slot<'_, Message> = action.into();
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
