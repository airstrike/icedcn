use iced::time::Instant;
use iced::{Element, Event, Point, Size, window};
use iced_test::{Simulator, simulator};
use icedcn::Theme;

use super::{Message, contributions, controls};

type Ui = Simulator<'static, Message, Theme, iced::Renderer>;

fn frame(ui: &mut Ui) {
    ui.draw(&Theme::light());
    ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
    ui.draw(&Theme::light());
}

#[test]
fn showcase_buttons_do_not_navigate_away() {
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(370., 600.),
        Element::from(controls()),
    );
    frame(&mut ui);
    for label in ["Button", "Secondary", "Outline", "Button Group"] {
        ui.click(label).expect("showcase button");
    }
    assert!(ui.into_messages().next().is_none());

    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(370., 600.),
        Element::from(contributions()),
    );
    frame(&mut ui);
    ui.click("View Full Report")
        .expect("reference report button");
    assert!(ui.find("Apr").is_ok());
    assert!(ui.find("May").is_err());
    assert!(ui.into_messages().next().is_none());
}

#[test]
fn alert_and_quick_actions_work_inside_the_showcase() {
    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(370., 600.),
        Element::from(controls()),
    );
    frame(&mut ui);
    ui.click("Alert Dialog").expect("alert trigger");
    ui = ui.rebuild(Element::from(controls()));
    frame(&mut ui);
    ui.find("Allow accessory to connect?")
        .expect("local alert content");
    ui.click("Allow").expect("local confirmation");
    assert!(ui.into_messages().next().is_none());

    let mut ui = Ui::with_size(
        Default::default(),
        Size::new(370., 600.),
        Element::from(controls()),
    );
    frame(&mut ui);
    let label = ui.find("Button Group").expect("split button").bounds();
    ui.point_at(Point::new(label.x + label.width + 22., label.center_y()));
    ui.simulate(simulator::click());
    ui = ui.rebuild(Element::from(controls()));
    frame(&mut ui);
    ui.find("Quick Actions").expect("local quick-actions menu");
    ui.click("Mark as Read").expect("menu action");
    assert!(ui.into_messages().next().is_none());
}
