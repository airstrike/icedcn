use browser::component::switch::SwitchPage;
use iced::keyboard::key::Named;
use iced::{Element, Event, Point, Rectangle, mouse};
use iced_test::core::widget::{Id, Operation, operation::Focusable};
use iced_test::{Simulator, simulator};
use icedcn::Theme;
use icedcn::widget::operation::control;
use icedcn::widget::{container, switch};
use std::any::Any;

#[derive(Default)]
struct Switches {
    values: Vec<bool>,
    tracks: Vec<Rectangle>,
}

impl Operation for Switches {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn Focusable) {
        self.tracks.push(bounds);
    }

    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Any) {
        let mut read = control::checked();
        read.custom(id, bounds, state);
        self.values.extend(read.get());
    }
}

fn controls<'a, M: 'a>(ui: &mut Simulator<'a, M, Theme, iced::Renderer>) -> Switches {
    let mut state = Switches::default();
    ui.operate(&mut state);
    state
}

#[test]
fn press_requires_a_release_in_the_enlarged_target() {
    let view: Element<'_, bool, Theme> = container(switch().on_checked_change(|it| it))
        .padding(24)
        .into();
    let mut ui = Simulator::new(view);
    let track = controls(&mut ui).tracks[0];
    let inside = Point::new(track.x - 8., track.center_y());
    let outside = Point::new(track.x - 16., track.center_y());
    let press = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
    let release = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
    ui.point_at(inside);
    ui.simulate([press.clone()]);
    assert_eq!(ui.drain().count(), 0);
    ui.point_at(outside);
    ui.simulate([release.clone()]);
    assert_eq!(controls(&mut ui).values, [false]);
    ui.simulate([press]);
    ui.point_at(inside);
    ui.simulate([release]);
    assert_eq!(ui.drain().count(), 0);
    ui.simulate(simulator::click());
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
}

#[test]
fn controlled_switch_waits_for_the_next_app_value() {
    let make = |checked| -> Element<'static, bool, Theme> {
        switch().checked(checked).on_checked_change(|it| it).into()
    };
    let mut ui = Simulator::new(make(false));
    let point = controls(&mut ui).tracks[0].center();
    ui.point_at(point);
    ui.simulate(simulator::click());
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
    assert_eq!(controls(&mut ui).values, [false]);
    let mut ui = ui.rebuild(make(true));
    assert_eq!(controls(&mut ui).values, [true]);
}

#[test]
fn card_description_and_track_each_toggle_exactly_once() {
    let page = SwitchPage::new(0);
    let mut ui = Simulator::new(page.preview("choice-card"));
    assert_eq!(controls(&mut ui).values, [false, true]);
    ui.click("Receive notifications when focus mode is enabled or disabled.")
        .unwrap();
    ui.draw(&Theme::light());
    assert_eq!(controls(&mut ui).values, [false, false]);
    let track = controls(&mut ui).tracks[1].center();
    ui.point_at(track);
    ui.simulate(simulator::click());
    assert_eq!(controls(&mut ui).values, [false, true]);
    ui.tap_key(Named::Space);
    assert_eq!(controls(&mut ui).values, [false, false]);
}

#[test]
fn touch_tracks_the_pressing_finger_and_cancels_a_lost_gesture() {
    use iced::touch::{self, Finger};
    let view: Element<'_, bool, Theme> = switch().on_checked_change(|value| value).into();
    let mut ui = Simulator::new(view);
    let position = controls(&mut ui).tracks[0].center();
    let id = Finger(1);
    let down = Event::Touch(touch::Event::FingerPressed {
        id,
        position,
        timestamp: None,
    });
    let up = Event::Touch(touch::Event::FingerLifted {
        id,
        position,
        timestamp: None,
    });
    ui.simulate([
        down.clone(),
        Event::Touch(touch::Event::FingerLifted {
            timestamp: None,
            id: Finger(2),
            position,
        }),
    ]);
    assert_eq!(ui.drain().count(), 0);
    ui.simulate([
        Event::Touch(touch::Event::FingerLost {
            id,
            position,
            timestamp: None,
        }),
        up.clone(),
    ]);
    assert_eq!(controls(&mut ui).values, [false]);
    ui.simulate([down, up]);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
}
