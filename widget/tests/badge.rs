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

#[test]
fn passive_hover_reports_state_without_joining_keyboard_traversal() {
    use std::cell::Cell;
    let observed = Cell::new(badge::Status::default());
    let content: Element<'_, (), Theme, Renderer> = badge(Space::new().width(24).height(16))
        .state_style(|_, status| {
            observed.set(status);
            badge::Style::default()
        })
        .into();
    let mut ui = simulator(content);
    ui.point_at(Point::new(20.0, 10.0));
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut second) = next.finish() {
        ui.operate(second.as_mut());
    }
    ui.draw(&Theme::light());
    assert!(observed.get().hovered);
    assert!(!observed.get().focused);
    assert!(!observed.get().interactive);
    assert!(
        ui.simulate(simulator::click())
            .iter()
            .all(|s| *s == event::Status::Ignored)
    );
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn pointer_and_keyboard_focus_have_distinct_paint_contexts() {
    use std::cell::Cell;
    let observed = Cell::new(badge::Status::default());
    let content: Element<'_, (), Theme, Renderer> = badge(Space::new().width(24).height(16))
        .on_press(())
        .invalid(true)
        .state_style(|_, status| {
            observed.set(status);
            badge::Style::default()
        })
        .into();
    let mut ui = simulator(content);
    ui.point_at(Point::new(20.0, 10.0));
    ui.simulate(simulator::click());
    assert_eq!(ui.drain().count(), 1);
    ui.draw(&Theme::light());
    assert!(observed.get().interactive && observed.get().invalid);
    assert!(!observed.get().focused);
    ui.point_at(Point::new(600.0, 300.0));
    ui.tap_key(Named::Enter);
    ui.draw(&Theme::light());
    assert!(observed.get().focused);
    assert!(!observed.get().hovered);
    assert_eq!(ui.drain().count(), 1);
}

#[test]
fn pointer_release_outside_cancels_and_disabled_rebuild_clears_press() {
    let mut ui = simulator(linked(false));
    let press = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
    let release = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
    ui.point_at(Point::new(20.0, 10.0));
    assert_eq!(ui.simulate([press.clone()]), [event::Status::Captured]);
    ui.point_at(Point::new(100.0, 10.0));
    ui.simulate([release.clone()]);
    assert_eq!(ui.drain().count(), 0);
    ui.point_at(Point::new(20.0, 10.0));
    ui.simulate([press]);
    ui = ui.rebuild(linked(true));
    ui.simulate([release.clone()]);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().count(), 0);
    ui = ui.rebuild(linked(false));
    ui.simulate([release]);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().count(), 0);
    ui.simulate(simulator::click());
    assert_eq!(ui.drain().count(), 1);
}

#[test]
fn last_style_builder_replaces_the_previous_callback() {
    use iced_widget::container;
    use std::cell::Cell;
    let stale_calls = Cell::new(0);
    let current_calls = Cell::new(0);
    let legacy: container::StyleFn<'_, Theme> = Box::new(|_| {
        stale_calls.set(stale_calls.get() + 1);
        container::Style::default()
    });
    let content: Element<'_, (), Theme, Renderer> = badge(Space::new().width(24).height(16))
        .style(legacy)
        .state_style(|_, _| {
            current_calls.set(current_calls.get() + 1);
            badge::Style::default()
        })
        .into();
    simulator(content).draw(&Theme::light());
    assert_eq!(stale_calls.get(), 0);
    assert!(current_calls.get() > 0);
    let content: Element<'_, (), Theme, Renderer> = badge(Space::new().width(24).height(16))
        .state_style(|_, _| {
            stale_calls.set(stale_calls.get() + 1);
            badge::Style::default()
        })
        .style(badge::Outline)
        .into();
    simulator(content).draw(&Theme::light());
    assert_eq!(stale_calls.get(), 0);
}

#[test]
fn touch_release_uses_touch_position_and_cancellation_does_not_activate() {
    use iced_core::touch;
    let mut ui = simulator(linked(false));
    let id = touch::Finger(1);
    ui.point_at(Point::new(600., 300.));
    ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            id,
            position: Point::new(20., 10.),
        }),
        Event::Touch(touch::Event::FingerLifted {
            id,
            position: Point::new(20., 10.),
        }),
    ]);
    assert_eq!(ui.drain().count(), 1);
    ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            id,
            position: Point::new(20., 10.),
        }),
        Event::Touch(touch::Event::FingerLost {
            id,
            position: Point::new(20., 10.),
        }),
        Event::Touch(touch::Event::FingerLifted {
            id,
            position: Point::new(20., 10.),
        }),
    ]);
    assert_eq!(ui.drain().count(), 0);
}
