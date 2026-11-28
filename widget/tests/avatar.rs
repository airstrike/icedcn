//! Avatar geometry and composition survive groups and disabled ancestors.
use std::cell::RefCell;
use std::rc::Rc;

use iced_core::image::Handle;
use iced_core::widget::{Id, Operation};
use iced_core::{Element, Point, Rectangle};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space, Widget};
use icedcn_core::Theme;
use icedcn_widget::{Compose, Inherited, Size, View, Weight, avatar, button, text};

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
fn group_converts_image_handles_and_accepts_mixed_macro_children() {
    let image = Handle::from_rgba(1, 1, vec![255, 0, 0, 255]);
    let from_images = avatar::group::<()>([image.clone()]);
    assert_eq!(from_images.count_geometry().diameter, 32.0);
    let mixed = avatar::group![image.clone(), avatar::<()>(image).size(avatar::Lg)];
    assert_eq!(mixed.count_geometry().diameter, 40.0);
    let empty: avatar::group::Group<'_, ()> = avatar::group![];
    assert_eq!(empty.count_geometry().diameter, 32.0);
}

#[test]
fn count_follows_any_named_member_and_small_wins() {
    for (sizes, expected) in [
        (
            [avatar::Size::from(avatar::Default), avatar::Lg.into()],
            40.0,
        ),
        ([avatar::Lg.into(), avatar::Sm.into()], 24.0),
        ([avatar::Sm.into(), avatar::Lg.into()], 24.0),
        ([48.0.into(), avatar::Default.into()], 32.0),
    ] {
        let group = avatar::group(sizes.map(|size| avatar::<()>(None).size(size))).count("+3");
        assert_eq!(group.count_geometry().diameter, expected);
        assert_eq!(group.count_geometry().font_size, 14.0);
        let mut ui = simulator(Element::from(group));
        let mut bounds = Bounds::default();
        ui.operate(&mut bounds);
        let count = bounds.0.last().expect("count container");
        assert_eq!(count.width, expected);
        assert_eq!(count.height, expected);
    }
}

#[test]
fn badged_group_keeps_footprint_and_eight_pixel_overlap() {
    let mut ui = simulator(Element::from(
        avatar::group([
            avatar::badge(avatar::<()>(None), avatar::dot()),
            avatar(None),
        ])
        .count("+3"),
    ));
    let mut bounds = Bounds::default();
    ui.operate(&mut bounds);
    let circles: Vec<_> = bounds.0.iter().filter(|b| b.width == 32.0).collect();
    assert_eq!(
        circles.iter().map(|b| b.x).collect::<Vec<_>>(),
        [0.0, 0.0, 24.0, 24.0, 48.0, 48.0]
    );
    assert!(bounds.0.iter().any(|b| *b
        == Rectangle {
            x: 22.0,
            y: 22.0,
            width: 10.0,
            height: 10.0
        }));
}

struct Probe {
    received: Rc<RefCell<Vec<Inherited>>>,
    explicit: Inherited,
}

impl Compose<()> for Probe {
    fn inherit(&mut self, from: &Inherited) {
        self.explicit = self.explicit.or(*from);
    }

    fn compose<'a>(self: Box<Self>) -> Element<'a, (), Theme, Renderer>
    where
        Self: 'a,
    {
        self.received.borrow_mut().push(self.explicit);
        Space::new().width(1).height(1).boxed()
    }
}

fn probe(received: &Rc<RefCell<Vec<Inherited>>>, explicit: Inherited) -> View<'static> {
    View::new(Probe {
        received: received.clone(),
        explicit,
    })
}

#[test]
fn group_badge_and_count_receive_props_without_overriding_explicit_children() {
    let received = Rc::new(RefCell::new(Vec::new()));
    let mut group: View<'_> = avatar::group([avatar::badge(
        avatar(None).fallback(probe(&received, Inherited::default().with_size(19.0))),
        probe(&received, Inherited::default()),
    )])
    .count(probe(&received, Inherited::default()))
    .into();
    group.inherit(
        &Inherited::default()
            .with_disabled(true)
            .with_weight(Weight::Bold),
    );
    let _ = group.compose();
    let props = received.borrow();
    assert_eq!(props.len(), 3);
    assert_eq!(
        props.iter().map(|p| p.size()).collect::<Vec<_>>(),
        [
            Some(Size::Custom(19.0)),
            Some(Size::Custom(8.0)),
            Some(text::size::SM)
        ]
    );
    assert!(props.iter().all(|p| p.is_disabled()
        && p.weight() == Some(Weight::Bold)
        && p.selectable() == Some(false)));
}

#[test]
fn small_badge_hides_content_without_shrinking_its_dot() {
    let received = Rc::new(RefCell::new(Vec::new()));
    let avatar = avatar::badge(
        avatar(None).size(avatar::Sm),
        probe(&received, Inherited::default()),
    );
    let mut ui = simulator(Element::from(avatar));
    assert!(received.borrow().is_empty());
    let mut bounds = Bounds::default();
    ui.operate(&mut bounds);
    assert!(bounds.0.iter().any(|b| *b
        == Rectangle {
            x: 16.0,
            y: 16.0,
            width: 8.0,
            height: 8.0
        }));
}

#[test]
fn disabled_fallback_stays_disabled_through_group_rebuilds() {
    fn view(disabled: bool) -> Element<'static, bool, Theme, Renderer> {
        let mut slot: View<'_, bool> = avatar::group([
            avatar(None).fallback(button(Space::new().width(12).height(12)).on_press(true))
        ])
        .into();
        slot.inherit(&Inherited::default().with_disabled(disabled));
        slot.compose()
    }
    for disabled in [false, true] {
        let mut ui = simulator(view(disabled));
        for _ in 0..2 {
            ui.point_at(Point::new(16.0, 16.0));
            ui.simulate(simulator::click());
            assert_eq!(ui.drain().count(), usize::from(!disabled));
            ui = ui.rebuild(view(disabled));
        }
    }
}

#[test]
fn avatar_trigger_opens_selects_and_reopens_its_dropdown() {
    use icedcn_widget::{column, dropdown_menu, menu};
    fn view() -> Element<'static, bool, Theme, Renderer> {
        dropdown_menu(
            |m| {
                m.trigger(
                    button(avatar(None).fallback("CN"))
                        .style(button::Ghost)
                        .square()
                        .rounded(),
                )
            },
            |_| column![menu::item(Space::new().width(60).height(20)).on_select(true)].into(),
        )
        .width(128)
        .align(dropdown_menu::Align::Start)
        .into()
    }
    let mut ui = simulator(view());
    for _ in 0..2 {
        ui.point_at(Point::new(16.0, 16.0));
        ui.simulate(simulator::click());
        ui.point_at(Point::new(50.0, 54.0));
        ui.simulate(simulator::click());
        assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
        ui = ui.rebuild(view());
        ui.point_at(Point::new(50.0, 54.0));
        ui.simulate(simulator::click());
        assert_eq!(ui.drain().count(), 0, "selection must close the popup");
    }
}
