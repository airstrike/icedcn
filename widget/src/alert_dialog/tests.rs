use std::any::Any;
use std::time::Duration;

use iced_core::keyboard::{self, key::Named};
use iced_core::time::Instant;
use iced_core::widget::operation::Focusable;
use iced_core::widget::{Id, Operation};
use iced_core::{Element, Event, Point, Rectangle, Size, touch, window};
use iced_test::simulator;
use iced_widget::{Renderer, Space};
use icedxdy_core::Theme;

use super::alert_dialog;
use crate::button;
use crate::operation::focus;

type Simulator = iced_test::Simulator<'static, (), Theme, Renderer>;

#[derive(Default)]
struct Indicators(Vec<(bool, bool)>);

impl Operation for Indicators {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn focusable(&mut self, _: Option<&Id>, _: Rectangle, control: &mut dyn Focusable) {
        self.0.push((control.is_focused(), false));
    }

    fn custom(&mut self, _: Option<&Id>, _: Rectangle, state: &mut dyn Any) {
        if let Some(visible) = state.downcast_ref::<focus::Visible>() {
            self.0.last_mut().expect("indicator follows its control").1 = visible.0;
        }
    }
}

fn view(default_open: bool) -> Element<'static, (), Theme, Renderer> {
    alert_dialog(
        |a| a.trigger(button(Space::new().width(80).height(16))),
        |a| {
            a.footer([
                a.cancel(button(Space::new().width(80).height(16))),
                a.action(button(Space::new().width(80).height(16))),
            ])
        },
    )
    .default_open(default_open)
    .into()
}

fn focused(ui: &mut Simulator, index: usize, visible: bool, total: usize) {
    let mut indicators = Indicators::default();
    ui.operate(&mut indicators);
    assert_eq!(indicators.0.len(), total);
    for (i, (is_focused, is_visible)) in indicators.0.into_iter().enumerate() {
        assert_eq!(is_focused, i == index);
        if is_focused {
            assert_eq!(is_visible, visible, "focus visibility at index {i}");
        }
    }
}

fn rebuild(ui: Simulator) -> Simulator {
    let mut ui = ui.rebuild(view(false));
    ui.draw(&Theme::light());
    ui
}

fn dismiss(mut ui: Simulator) -> Simulator {
    ui.tap_key(Named::Escape);
    ui = rebuild(ui);
    ui.simulate([Event::Window(window::Event::RedrawRequested(
        Instant::now() + Duration::from_millis(180),
    ))]);
    rebuild(ui)
}

#[test]
fn pointer_and_touch_opening_preserve_focus_without_a_keyboard_ring() {
    for width in [1440., 390.] {
        for touch in [false, true] {
            let mut ui =
                Simulator::with_size(Default::default(), Size::new(width, 900.), view(false));
            ui.draw(&Theme::light());
            let position = Point::new(10., 10.);
            ui.point_at(position);
            if touch {
                ui.simulate([
                    Event::Touch(touch::Event::FingerPressed {
                        id: touch::Finger(0),
                        position,
                    }),
                    Event::Touch(touch::Event::FingerLifted {
                        id: touch::Finger(0),
                        position,
                    }),
                ]);
            } else {
                ui.simulate(simulator::click());
            }
            ui = rebuild(ui);
            focused(&mut ui, 0, false, 2);
            ui = rebuild(ui);
            focused(&mut ui, 0, false, 2);

            ui.tap_key(Named::Tab);
            focused(&mut ui, 1, true, 2);
            let mut previous = simulator::press_key(Named::Tab, None);
            if let Event::Keyboard(keyboard::Event::KeyPressed { modifiers, .. }) = &mut previous {
                *modifiers = keyboard::Modifiers::SHIFT;
            }
            ui.simulate([previous]);
            focused(&mut ui, 0, true, 2);
            ui = dismiss(ui);
            focused(&mut ui, 0, true, 1);

            for key in [Named::Enter, Named::Space] {
                ui.tap_key(key);
                ui = rebuild(ui);
                focused(&mut ui, 0, true, 2);
                ui = dismiss(ui);
                focused(&mut ui, 0, true, 1);
            }

            ui.point_at(position);
            ui.simulate(simulator::click());
            ui = rebuild(ui);
            focused(&mut ui, 0, false, 2);
        }
    }
}

#[test]
fn default_open_keeps_visible_initial_focus() {
    let mut ui = Simulator::with_size(Default::default(), Size::new(1440., 900.), view(true));
    ui.draw(&Theme::light());
    focused(&mut ui, 0, true, 2);
}
