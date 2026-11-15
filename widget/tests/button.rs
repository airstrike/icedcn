//! Regressions for Button's public layout and event contract.
use iced_core::keyboard::key::Named;
use iced_core::widget::operation::{Outcome, focusable};
use iced_core::widget::{Id, Operation};
use iced_core::{Element, Event, Length, Point, Size, mouse, window};
use iced_test::core::Settings;
use iced_test::{Simulator, selector, simulator};
use iced_widget::{Renderer, Space, Widget};
use icedcn_core::{Theme, theme};
use icedcn_widget::Size::{Default, Lg, Sm, Xs};
use icedcn_widget::{Inherited, Slot, button, container};

type Preview<'a> = Simulator<'a, (), Theme, Renderer>;

fn next(ui: &mut Preview<'_>) {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
}

#[test]
fn size_border_box_and_macro_gaps_use_the_source_dimensions() {
    for size in [Xs, Sm, Default, Lg] {
        let b = button![
            Space::new().width(10).height(12),
            Space::new().width(10).height(12)
        ]
        .size(size);
        let content: Element<'_, (), Theme, Renderer> = b.into();
        let mut ui: Preview<'_> =
            Simulator::with_size(Settings::default(), Size::new(300., 100.), content);
        next(&mut ui);
        let bounds = ui
            .find(selector::is_focused())
            .expect("enabled Button")
            .bounds();
        let padding = if size == Xs { 18.0 } else { 22.0 };
        assert_eq!(bounds.width, 20.0 + size.gap() + padding);
        assert_eq!(bounds.height, size.height());
    }
}

#[test]
fn stock_style_keeps_legacy_layout_and_square_dimensions_stay_exact() {
    let stock: iced_widget::button::StyleFn<'_, Theme> = theme::button::Outline.into();
    let b = button(Space::new().width(10).height(12)).style(stock);
    let mut ui: Preview<'_> =
        Simulator::with_size(Settings::default(), Size::new(300., 100.), Element::from(b));
    next(&mut ui);
    assert_eq!(
        ui.find(selector::is_focused())
            .expect("stock button")
            .bounds()
            .width,
        30.0
    );
    for size in [Xs, Sm, Default, Lg] {
        let b = button(Space::new().width(12).height(12))
            .size(size)
            .square();
        let content: Element<'_, (), Theme, Renderer> = b.into();
        let mut ui: Preview<'_> =
            Simulator::with_size(Settings::default(), Size::new(300., 100.), content);
        next(&mut ui);
        let bounds = ui.find(selector::is_focused()).expect("square").bounds();
        assert_eq!(bounds.size(), Size::new(size.height(), size.height()));
    }
}

#[test]
fn intrinsic_button_centers_its_child_line_box_at_each_size() {
    for size in [Xs, Sm, Default, Lg] {
        let id = Id::unique();
        let label = container(Space::new().width(40).height(16)).id(id.clone());
        let b = button(label).size(size);
        let mut ui: Preview<'_> =
            Simulator::with_size(Settings::default(), Size::new(300., 100.), Element::from(b));
        next(&mut ui);
        let outer = ui.find(selector::is_focused()).expect("button").bounds();
        let inner = ui.find(id).expect("label").bounds();
        assert_eq!(inner.height, 16.);
        assert_eq!(inner.center_y(), outer.center_y());
    }
}

#[test]
fn explicit_width_centers_nova_content_at_roomy_and_narrow_widths() {
    for available in [300.0, 260.0] {
        for width in [Length::Fixed(200.0), Length::FillPortion(1)] {
            let id = Id::unique();
            let content = container(Space::new().width(20).height(12))
                .width(20)
                .height(12)
                .id(id.clone());
            let b = button(content).width(width);
            let mut ui: Preview<'_> = Simulator::with_size(
                Settings::default(),
                Size::new(available, 100.0),
                Element::from(b),
            );
            next(&mut ui);
            let outer = ui.find(selector::is_focused()).expect("button").bounds();
            let inner = ui.find(id).expect("child").bounds();
            assert_eq!(inner.size(), Size::new(20.0, 12.0));
            assert_eq!(inner.center(), outer.center());
        }
    }
}

#[test]
fn a_grid_centers_the_label_when_it_stretches_an_intrinsic_button() {
    let id = Id::unique();
    let label = container(Space::new().width(20).height(12)).id(id.clone());
    let button: Element<'_, (), Theme, Renderer> = button(label).into();
    let grid = iced_widget::grid([button])
        .columns(1)
        .height(Length::Shrink);
    let mut ui: Preview<'_> =
        Simulator::with_size(Settings::default(), Size::new(260., 100.), grid.boxed());
    next(&mut ui);
    let outer = ui.find(selector::is_focused()).expect("button").bounds();
    let inner = ui.find(id).expect("label").bounds();
    assert_eq!(outer.width, 260.);
    assert_eq!(inner.width, 20.);
    assert_eq!(inner.center(), outer.center());
}

#[test]
fn pointer_keyboard_disabled_and_cancellation_preserve_messages() {
    for disabled in [false, true] {
        let mut content: Slot<'_, ()> = button(Space::new().width(20).height(12))
            .on_press(())
            .into();
        content.inherit(&Inherited::default().with_disabled(disabled));
        let mut ui: Preview<'_> = Simulator::with_size(
            Settings::default(),
            Size::new(300., 100.),
            content.compose(),
        );
        ui.point_at(Point::new(10., 10.));
        for _ in 0..2 {
            ui.simulate(simulator::click());
            assert_eq!(ui.drain().count(), usize::from(!disabled));
        }
        ui.point_at(Point::new(200., 90.));
        ui.simulate(simulator::click());
        assert_eq!(ui.drain().count(), 0);
        next(&mut ui);
        for key in [Named::Enter, Named::Space] {
            ui.tap_key(key);
            assert_eq!(ui.drain().count(), usize::from(!disabled));
        }
        ui.point_at(Point::new(10., 10.));
        ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
        ui.point_at(Point::new(200., 90.));
        ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
            mouse::Button::Left,
        ))]);
        assert_eq!(ui.drain().count(), 0);
        ui.point_at(Point::new(10., 10.));
        ui.simulate([
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
            Event::Window(window::Event::Unfocused),
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
        ]);
        assert_eq!(ui.drain().count(), 0);
    }
}
