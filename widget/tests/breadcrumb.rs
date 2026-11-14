//! Breadcrumb activation, inheritance and responsive layout through public APIs.
use std::cell::RefCell;
use std::rc::Rc;

use iced_core::keyboard::key::Named;
use iced_core::widget::Id;
use iced_core::widget::operation::{self, Operation, Outcome, focusable};
use iced_core::{Element, Event, Point, Size, mouse};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space, Widget};
use icedxdy_core::Theme;
use icedxdy_widget::{
    Compose, Inherited, Slot, Weight, breadcrumb, button, container, dropdown_menu, popover, row,
};

type Preview = iced_test::Simulator<'static, u32, Theme, Renderer>;

fn focus(ui: &mut Preview) -> focusable::Count {
    let mut op = focusable::count();
    ui.operate(&mut operation::black_box(&mut op));
    match op.finish() {
        Outcome::Some(value) => value,
        _ => panic!("focus count completes"),
    }
}

fn next(ui: &mut Preview, backwards: bool) {
    let mut op: Box<dyn Operation> = if backwards {
        Box::new(focusable::focus_previous::<()>())
    } else {
        Box::new(focusable::focus_next::<()>())
    };
    ui.operate(op.as_mut());
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
}

fn trail(disabled: bool) -> Element<'static, u32, Theme, Renderer> {
    let mut content: Slot<'_, u32> = breadcrumb([
        breadcrumb::link(Space::new().width(40).height(20), 1),
        breadcrumb::item(
            button(Space::new().width(40).height(20))
                .height(20)
                .padding(0)
                .on_press(2),
        ),
        breadcrumb::ellipsis(Some(3)),
        breadcrumb::page(Space::new().width(40).height(20)),
        breadcrumb::ellipsis(None),
    ])
    .into();
    content.inherit(&Inherited::default().with_disabled(disabled));
    content.compose()
}

#[test]
fn links_publish_on_release_cancel_outside_and_keep_keyboard_focus_after_rebuild() {
    let mut ui = simulator(trail(false));
    ui.point_at(Point::new(10., 10.));
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    assert_eq!(ui.drain().count(), 0);
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [1]);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(Point::new(700., 700.));
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    assert_eq!(ui.drain().count(), 0);
    ui = ui.rebuild(trail(false));
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [1]);
    assert_eq!(
        focus(&mut ui).total,
        3,
        "current page and static ellipsis are not focus targets"
    );
    next(&mut ui, false);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [2]);
    next(&mut ui, false);
    ui.tap_key(Named::Space);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [3]);
    next(&mut ui, true);
    assert_eq!(focus(&mut ui).focused, Some(1));
}

#[test]
fn disabled_ancestor_suppresses_links_custom_buttons_and_ellipsis_after_rebuild() {
    let mut ui = simulator(trail(false));
    ui.point_at(Point::new(10., 10.));
    ui.simulate(simulator::click());
    ui.drain().for_each(drop);
    ui = ui.rebuild(trail(true));
    assert_eq!(focus(&mut ui).total, 0);
    for x in [10., 75., 140.] {
        ui.point_at(Point::new(x, 10.));
        ui.simulate(simulator::click());
        ui.tap_key(Named::Enter);
    }
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn constrained_trail_wraps_whole_items_with_six_pixel_row_gaps() {
    let first = Id::unique();
    let last = Id::unique();
    let content: Element<'_, u32, Theme, Renderer> = breadcrumb([
        breadcrumb::link(
            container(Space::new().width(90).height(20)).id(first.clone()),
            1,
        ),
        breadcrumb::page(container(Space::new().width(90).height(20)).id(last.clone())),
    ])
    .into();
    let mut ui =
        iced_test::Simulator::with_size(Default::default(), Size::new(220., 100.), content);
    assert_eq!(
        ui.find(first.clone()).unwrap().bounds().y,
        ui.find(last.clone()).unwrap().bounds().y
    );
    ui = ui.resize(Size::new(150., 100.));
    let a = ui.find(first).unwrap().bounds();
    let b = ui.find(last).unwrap().bounds();
    assert_eq!(b.y - a.y, 26.);
    assert!(b.x + b.width <= 150.);
}

struct Recorded(Rc<RefCell<Inherited>>, Inherited);

impl Compose<u32> for Recorded {
    fn inherit(&mut self, from: &Inherited) {
        self.1 = self.1.or(*from);
    }
    fn compose<'a>(self: Box<Self>) -> Element<'a, u32, Theme, Renderer>
    where
        Self: 'a,
    {
        *self.0.borrow_mut() = self.1;
        Space::new().width(10).height(10).boxed()
    }
}

#[test]
fn typography_cascades_without_overwriting_explicit_child_values() {
    let inherited = Inherited::default()
        .with_size(18.0)
        .with_weight(Weight::Bold)
        .with_line_height(24.);
    let observed = Rc::new(RefCell::new(Inherited::default()));
    let child = Slot::new(Recorded(
        observed.clone(),
        Inherited::default().with_size(12.0),
    ));
    let mut content: Slot<'_, u32> = breadcrumb([breadcrumb::page(child)]).into();
    content.inherit(&inherited);
    let _ = content.compose();
    let actual = *observed.borrow();
    assert_eq!(actual.size().unwrap().text_size(), 12.);
    assert_eq!(actual.weight(), Some(Weight::Bold));
    assert_eq!(actual.line_height(), Some(24.));
}

#[test]
fn parts_compose_in_controls_and_preserve_disabled_inheritance() {
    let view = |disabled| {
        let mut content: Slot<'_, u32> = row![
            breadcrumb::link(Space::new().width(40).height(20), 1),
            breadcrumb::page(Space::new().width(40).height(20)),
            button(breadcrumb::ellipsis(None))
                .height(28)
                .square()
                .on_press(2),
        ]
        .into();
        content.inherit(&Inherited::default().with_disabled(disabled));
        content.compose()
    };
    let mut ui = simulator(view(false));
    assert_eq!(focus(&mut ui).total, 2);
    next(&mut ui, false);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [1]);
    next(&mut ui, false);
    ui.tap_key(Named::Space);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [2]);
    ui = ui.rebuild(view(true));
    assert_eq!(focus(&mut ui).total, 0);
    ui.tap_key(Named::Enter);
    ui.tap_key(Named::Space);
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn arrow_down_is_dropdown_owned_and_respects_focus_and_disabling() {
    let menu = |disabled| {
        let mut content: Slot<'_, u32> = dropdown_menu(
            |m| m.trigger(button(Space::new().width(40).height(20)).on_press(77)),
            |_| Space::new().width(40).height(20).into(),
        )
        .on_open_change(|open| if open { 10 } else { 11 })
        .into();
        content.inherit(&Inherited::default().with_disabled(disabled));
        content.compose()
    };
    let mut ui = simulator(menu(false));
    ui.tap_key(Named::ArrowDown);
    assert_eq!(ui.drain().count(), 0, "unfocused trigger is inert");
    next(&mut ui, false);
    ui.tap_key(Named::ArrowDown);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [10],
        "opening does not press the child"
    );
    ui.tap_key(Named::Escape);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [11]);
    ui = ui.rebuild(menu(true));
    next(&mut ui, false);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().count(), 0, "disabled trigger is inert");

    let content: Element<'_, u32, Theme, Renderer> = popover(
        |p| p.trigger(button(Space::new().width(40).height(20)).on_press(77)),
        |_| Space::new().width(40).height(20).into(),
    )
    .on_open_change(|_| 10)
    .into();
    let mut ui = simulator(content);
    next(&mut ui, false);
    ui.tap_key(Named::ArrowDown);
    assert_eq!(
        ui.drain().count(),
        0,
        "ordinary Popover does not claim ArrowDown"
    );
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        [77, 10],
        "child handler and owner still fire"
    );

    let content: Element<'_, u32, Theme, Renderer> = button(Space::new().width(40).height(20))
        .on_press(77)
        .into();
    let mut ui = simulator(content);
    next(&mut ui, false);
    ui.tap_key(Named::ArrowDown);
    assert_eq!(
        ui.drain().count(),
        0,
        "ordinary Button does not claim ArrowDown"
    );
    ui.tap_key(Named::Space);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [77]);
}
