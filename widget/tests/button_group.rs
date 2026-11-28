//! Joining keeps the normal composition cascade and action ownership.
use iced_core::widget::operation::focusable;
use iced_core::widget::{Id, Operation};
use iced_core::{Element, Point, Rectangle};
use iced_test::{Simulator, simulator};
use iced_widget::{Renderer, Space, Widget};
use icedcn_core::Theme;
use icedcn_widget::button::group;
use icedcn_widget::{
    Compose, Inherited, Size, View, Weight, button, input, input_group, select, tooltip,
};
use std::cell::RefCell;
use std::rc::Rc;

struct Probe(Rc<RefCell<Inherited>>);
impl Compose<()> for Probe {
    fn inherit(&mut self, inherited: &Inherited) {
        *self.0.borrow_mut() = *inherited;
    }
    fn compose<'a>(self: Box<Self>) -> Element<'a, (), Theme, Renderer>
    where
        Self: 'a,
    {
        Space::new().width(12).height(12).boxed()
    }
}
#[derive(Default)]
struct Controls(Vec<Rectangle>);
impl Operation for Controls {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn focusable::Focusable) {
        self.0.push(bounds);
    }
}
#[test]
fn nested_mapped_slots_and_tooltips_preserve_the_cascade() {
    let received = Rc::new(RefCell::new(Inherited::default()));
    let mut slot: View<'_> = button::group![button::group![tooltip(
        View::new(Probe(received.clone())),
        "Hint"
    )]]
    .into();
    slot.inherit(
        &Inherited::default()
            .with_size(Size::Lg)
            .with_weight(Weight::Bold)
            .with_disabled(true),
    );
    let _ui = Simulator::new(slot.map(|()| ()).compose());
    assert_eq!(received.borrow().size(), Some(Size::Lg));
    assert_eq!(received.borrow().weight(), Some(Weight::Bold));
    assert!(received.borrow().is_disabled());
}
#[test]
fn inherited_disabled_reaches_buttons_inputs_select_and_input_group() {
    let mut slot: View<'_, bool> = button::group![
        button("Action").on_press(true),
        input("Input").width(90),
        select([select::item("one", "One")])
            .default_value("one")
            .on_value_change(|_| true),
        input_group(input("Message").on_input(|_| true)),
    ]
    .width(400)
    .into();
    slot.inherit(&Inherited::default().with_disabled(true));
    let mut ui = Simulator::new(slot.compose());
    let mut controls = Controls::default();
    ui.operate(&mut controls);
    assert!(
        controls.0.is_empty(),
        "disabled descendants are skipped by focus traversal"
    );
    ui.point_at(Point::new(10., 12.));
    ui.simulate(simulator::click());
    ui.typewrite("ignored");
    assert_eq!(ui.drain().count(), 0);
}
#[test]
fn separator_axis_and_text_are_independent_public_parts() {
    let element: Element<'_, (), Theme, Renderer> = button::group![
        group::text("Prefix"),
        group::separator().orientation(group::Horizontal),
        button("Action").width(120),
    ]
    .orientation(group::Vertical)
    .into();
    let mut ui = Simulator::new(element);
    let mut controls = Controls::default();
    ui.operate(&mut controls);
    assert_eq!(controls.0.len(), 1);
    assert_eq!(controls.0[0].width, 120.);
    assert_eq!(controls.0[0].y, 33.);
}

#[test]
fn adjoining_actions_keep_separate_pointer_and_keyboard_targets() {
    use iced_core::keyboard::key::Named;
    use iced_core::widget::operation::Outcome;
    use iced_core::{Event, mouse};

    let group = button::group![
        button("Left").width(60).on_press("left"),
        button("Right").width(60).on_press("right"),
    ];
    let mut ui = Simulator::new(Element::<_, Theme, Renderer>::from(group));
    for (x, expected) in [(59., "left"), (61., "right"), (61., "right")] {
        ui.point_at(Point::new(x, 10.));
        let _ = ui.simulate(simulator::click());
        assert_eq!(ui.drain().collect::<Vec<_>>(), [expected]);
    }
    ui.point_at(Point::new(20., 10.));
    let _ = ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(Point::new(80., 10.));
    let _ = ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    assert!(
        ui.drain().next().is_none(),
        "crossing the seam cancels the press"
    );
    ui.operate(&mut focusable::unfocus::<()>());
    for expected in ["left", "right"] {
        let mut next = focusable::focus_next::<()>();
        ui.operate(&mut next);
        if let Outcome::Chain(mut next) = next.finish() {
            ui.operate(next.as_mut());
        }
        for key in [Named::Enter, Named::Space] {
            ui.tap_key(key);
            assert_eq!(ui.drain().collect::<Vec<_>>(), [expected]);
        }
    }
}
