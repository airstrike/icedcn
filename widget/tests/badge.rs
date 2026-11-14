//! Linked badges participate in keyboard focus traversal and activation.
use iced_core::keyboard::key::Named;
use iced_core::widget::operation::{Outcome, focusable};
use iced_core::widget::{Id, Operation};
use iced_core::{Element, Event, Point, Rectangle, event, mouse};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;
use icedxdy_widget::{Inherited, Slot, badge};

#[test]
fn keyboard_activates_link_without_hover() {
    let element: Element<'_, _, Theme, Renderer> =
        badge("Open Link").icon_end().on_press(true).into();
    let mut ui = simulator(element);
    let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(600.0, 300.0),
    })]);
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut second) = next.finish() {
        ui.operate(second.as_mut());
    }
    let _ = ui.simulate([simulator::press_key(Named::Enter, None)]);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [true]);
}

#[derive(Default)]
struct Bounds(Vec<Rectangle>);

impl Operation for Bounds {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn container(&mut self, _: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
        self.0.push(bounds);
    }
}

#[test]
fn border_space_and_icon_insets_keep_the_twenty_pixel_box() {
    for linked in [false, true] {
        for (start, end, width) in [
            (false, false, 42.0),
            (true, false, 40.0),
            (true, true, 38.0),
        ] {
            // The slightly taller child reproduces the old icon line-box
            // regression without needing fonts in the null renderer.
            let mut content = badge(Space::new().width(24).height(16.5));
            if start {
                content = content.icon_start();
            }
            if end {
                content = content.icon_end();
            }
            if linked {
                content = content.on_press(());
            }
            let element: Element<'_, (), Theme, Renderer> = content.into();
            let mut ui = simulator(element);
            let mut bounds = Bounds::default();
            ui.operate(&mut bounds);
            let outer = bounds.0.first().expect("badge bounds");
            assert_eq!(outer.width, width);
            assert_eq!(outer.height, 20.0);
        }
    }
}

fn linked(disabled: bool) -> Element<'static, (), Theme, Renderer> {
    let mut content: Slot<'_, ()> = badge(Space::new().width(24).height(16)).on_press(()).into();
    content.inherit(&Inherited::default().with_disabled(disabled));
    content.compose()
}

#[test]
fn link_border_hitbox_and_disabled_state_survive_rebuild() {
    for disabled in [false, true] {
        let mut ui = simulator(linked(disabled));
        // The original 40px box missed the source badge's last two pixels.
        for _ in 0..2 {
            ui.point_at(Point::new(40.5, 10.0));
            ui.simulate(simulator::click());
            assert_eq!(ui.drain().count(), usize::from(!disabled));
            ui = ui.rebuild(linked(disabled));
        }
        // Clicking already focused the enabled link. Rebuild must retain it;
        // advancing to the next focus target here would leave the only badge.
        ui.point_at(Point::new(600.0, 300.0));
        ui.tap_key(Named::Enter);
        assert_eq!(ui.drain().count(), usize::from(!disabled));
    }
}

#[test]
fn decorative_badge_does_not_capture_pointer_presses() {
    let element: Element<'_, (), Theme, Renderer> = badge(Space::new().width(24).height(16)).into();
    let mut ui = simulator(element);
    ui.point_at(Point::new(21.0, 10.0));
    for status in ui.simulate(simulator::click()) {
        assert_eq!(status, event::Status::Ignored);
    }
    assert_eq!(ui.drain().count(), 0);
}
