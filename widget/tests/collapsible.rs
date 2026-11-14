//! Disclosure defaults, disabled roots and retained state across rebuilds.
use iced_core::keyboard::key::Named;
use iced_core::widget::operation::{Operation, Outcome, focusable};
use iced_core::{Element, Point};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::{Inherited, Slot, button, collapsible, column};

type Ui = iced_test::Simulator<'static, Message, Theme, Renderer>;
#[derive(Debug, Clone, PartialEq)]
enum Message {
    Toggle(bool),
    Child,
}

fn disclosure(open: Option<bool>, default_open: bool, disabled: bool) -> Slot<'static, Message> {
    let root = collapsible(|c| {
        column![
            c.trigger(
                button(Space::new().width(80).height(18))
                    .width(100)
                    .height(32)
            ),
            c.content(
                button(Space::new().width(80).height(18))
                    .width(100)
                    .height(32)
                    .on_press(Message::Child)
            ),
        ]
        .into()
    })
    .default_open(default_open)
    .disabled(disabled)
    .on_open_change(Message::Toggle);
    match open {
        Some(open) => root.open(open).into(),
        None => root.into(),
    }
}
fn click(ui: &mut Ui, y: f32) {
    ui.point_at(Point::new(10., y));
    ui.simulate(simulator::click());
}
fn next(ui: &mut Ui) {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
}

#[test]
fn root_disabled_blocks_pointer_and_keyboard_but_preserves_open_content_controls() {
    let mut ui = simulator(disclosure(None, true, true).compose());
    click(&mut ui, 16.);
    assert_eq!(ui.drain().count(), 0);
    // The disabled trigger is skipped by focus traversal.
    next(&mut ui);
    ui.tap_key(Named::Space);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Child]);
    click(&mut ui, 48.);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Child]);
    ui = ui.rebuild(disclosure(None, false, false).compose());
    click(&mut ui, 16.);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Toggle(false)]);
    click(&mut ui, 48.);
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn default_only_seeds_once_and_controlled_rebuilds_display_the_supplied_value() {
    let mut ui = simulator(disclosure(None, true, false).compose());
    ui = ui.rebuild(disclosure(None, false, false).compose());
    click(&mut ui, 48.);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Child]);
    click(&mut ui, 16.);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Toggle(false)]);
    ui = ui.rebuild(disclosure(None, true, false).compose());
    click(&mut ui, 48.);
    assert_eq!(ui.drain().count(), 0);
    ui = ui.rebuild(disclosure(Some(true), false, false).compose());
    click(&mut ui, 16.);
    click(&mut ui, 48.);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [Message::Toggle(false), Message::Child]
    );
    ui = ui.rebuild(disclosure(Some(false), true, false).compose());
    click(&mut ui, 48.);
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn keyboard_toggles_once_and_inherited_disabled_cannot_be_overridden() {
    let mut ui = simulator(disclosure(None, false, false).compose());
    next(&mut ui);
    for (key, expected) in [
        (Named::Space, true),
        (Named::Enter, false),
        (Named::Enter, true),
    ] {
        ui.tap_key(key);
        assert_eq!(ui.drain().collect::<Vec<_>>(), [Message::Toggle(expected)]);
    }
    let mut root = disclosure(None, true, false);
    root.inherit(&Inherited::default().with_disabled(true));
    let element: Element<'_, Message, Theme, Renderer> = root.compose();
    ui = ui.rebuild(element);
    click(&mut ui, 16.);
    click(&mut ui, 48.);
    next(&mut ui);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().count(), 0);
}
