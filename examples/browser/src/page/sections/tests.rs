use super::*;
use iced::Widget as _;
use iced::widget::mouse_area;
use iced::{Point, time::Instant};
use iced_test::core::renderer::Headless;
use iced_test::core::shell::{Bus, Waker};
use iced_test::futures::futures::executor::block_on;
use iced_widget::Space;

#[test]
fn a_hidden_toc_resumes_at_the_current_viewport_and_reports_each_change_once() {
    let renderer = block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
    let items = (0..3)
        .map(|_| Space::new().width(Fill).height(40).boxed())
        .collect();
    let mut sections = Sections::new(items, 7, false);
    let mut tree = Tree::new(&sections);
    sections.diff(&mut tree);
    sections.layout(
        &mut tree,
        &renderer,
        &layout::Limits::new(Size::ZERO, Size::new(100., 400.)),
    );
    let layout = Layout::new(tree.size);
    let redraw = Event::Window(window::Event::RedrawRequested(Instant::now()));
    let mut bus = Bus::new();
    let mut update = |sections: &mut Sections<'_>, tree: &mut Tree, event: &Event, y: f32| {
        let viewport = Rectangle::new(Point::new(0., y), Size::new(100., 24.));
        sections.update(
            tree,
            event,
            layout,
            mouse::Cursor::Unavailable,
            &renderer,
            &mut Shell::new(&window::Headless, Waker::noop(), &mut bus),
            &viewport,
        );
        bus.drain()
            .map(|(m, _)| match m {
                Message::Page(7, page::Message::VisibleSections(indices)) => indices,
                _ => panic!("unexpected section message"),
            })
            .collect::<Vec<_>>()
    };
    assert!(update(&mut sections, &mut tree, &redraw, 176.).is_empty());
    sections.notify = true;
    sections.diff(&mut tree);
    assert_eq!(
        update(&mut sections, &mut tree, &redraw, 176.),
        [BTreeSet::from([1, 2])]
    );
    assert!(update(&mut sections, &mut tree, &redraw, 176.).is_empty());
    assert_eq!(
        update(&mut sections, &mut tree, &redraw, 0.),
        [BTreeSet::from([0])]
    );
    sections.notify = false;
    sections.diff(&mut tree);
    assert!(update(&mut sections, &mut tree, &redraw, 176.).is_empty());
    sections.notify = true;
    sections.diff(&mut tree);
    assert_eq!(
        update(&mut sections, &mut tree, &redraw, 176.),
        [BTreeSet::from([1, 2])]
    );

    let viewport = Rectangle::with_size(tree.size);
    let cursor = mouse::Cursor::Available(Point::new(10., 97.));
    assert_eq!(pointer_rows(&tree, layout, cursor, &viewport), 1..2);
    let gutter = mouse::Cursor::Available(Point::new(10., 70.));
    assert!(pointer_rows(&tree, layout, gutter, &viewport).is_empty());
    let press = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
    let release = Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left));
    update(&mut sections, &mut tree, &press, 0.);
    assert_eq!(pointer_rows(&tree, layout, cursor, &viewport), 0..3);
    update(&mut sections, &mut tree, &release, 0.);
    assert_eq!(pointer_rows(&tree, layout, cursor, &viewport), 1..2);
    assert_eq!(
        pointer_rows(&tree, layout, cursor.levitate(), &viewport),
        0..3
    );
}

#[test]
fn hovering_a_nested_wheel_handler_captures_its_event_and_gutters_pass_through() {
    let renderer = block_on(Renderer::new(Default::default(), None)).expect("headless renderer");
    let items = (0..3)
        .map(|index| {
            mouse_area(Space::new().width(Fill).height(40))
                .on_scroll(move |_| Message::SelectPage(index))
                .boxed()
        })
        .collect();
    let mut sections = Sections::new(items, 0, false);
    let mut tree = Tree::new(&sections);
    sections.diff(&mut tree);
    sections.layout(
        &mut tree,
        &renderer,
        &layout::Limits::new(Size::ZERO, Size::new(100., 400.)),
    );
    let layout = Layout::new(tree.size);
    let viewport = Rectangle::with_size(tree.size);
    let step = Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Pixels { x: 20., y: 0. },
    });
    for (y, expected) in [(96., Some(1)), (70., None), (196., Some(2))] {
        let mut bus = Bus::new();
        let mut shell = Shell::new(&window::Headless, Waker::noop(), &mut bus);
        sections.update(
            &mut tree,
            &step,
            layout,
            mouse::Cursor::Available(Point::new(10., y)),
            &renderer,
            &mut shell,
            &viewport,
        );
        assert_eq!(shell.is_event_captured(), expected.is_some());
        let messages = bus
            .into_iter()
            .map(|m| match m {
                Message::SelectPage(index) => index,
                _ => panic!("unexpected wheel message"),
            })
            .collect::<Vec<_>>();
        assert_eq!(messages, expected.into_iter().collect::<Vec<_>>());
    }
}
