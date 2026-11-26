//! Public Checkbox input contracts, including linked and inherited state.
use iced_core::keyboard::key::Named;
use iced_core::widget::Operation;
use iced_core::widget::operation::{Outcome, black_box, focusable};
use iced_core::{Element, Event, Point, mouse};
use iced_test::simulator::{self, simulator};
use iced_widget::{Renderer, Space};
use icedcn_core::Theme;
use icedcn_widget::operation::control;
use icedcn_widget::{Inherited, Slot, checkbox, column, field};

type Ui = iced_test::Simulator<'static, bool, Theme, Renderer>;

fn checked(ui: &mut Ui) -> bool {
    let mut read = control::checked();
    ui.operate(&mut read);
    read.get().expect("checkbox control")
}

fn advance(ui: &mut Ui, reverse: bool) {
    let mut operation: Box<dyn iced_core::widget::Operation> = if reverse {
        Box::new(focusable::focus_previous::<()>())
    } else {
        Box::new(focusable::focus_next::<()>())
    };
    loop {
        ui.operate(operation.as_mut());
        if let Outcome::Chain(next) = operation.finish() {
            operation = next;
        } else {
            break;
        }
    }
}

fn focus_count(ui: &mut Ui) -> focusable::Count {
    let mut count = focusable::count();
    ui.operate(&mut black_box(&mut count));
    let Outcome::Some(count) = count.finish() else {
        panic!("focus count");
    };
    count
}

fn element() -> Element<'static, bool, Theme, Renderer> {
    checkbox().on_checked_change(|value| value).into()
}

#[cfg(any(feature = "wgpu", feature = "vello-cpu"))]
#[test]
fn inline_label_reports_its_drawn_bounds_for_text_selection() {
    use iced_core::text::Target;
    use iced_core::widget::operation::text;
    use iced_test::selector;

    let mut ui = simulator(Element::from(
        checkbox()
            .label("Clip content")
            .width(120)
            .on_checked_change(|value| value),
    ));
    advance(&mut ui, false);
    let control = ui.find(selector::is_focused()).unwrap().bounds();
    let label = ui.find("Clip content").unwrap().bounds();
    assert_eq!(label.x, control.x + 16. + 8.);
    assert_eq!(label.x + label.width, control.x + control.width);
    assert_eq!(label.center_y(), control.center_y());

    let start = Point::new(label.x + 6., label.center_y());
    ui.operate(&mut text::select(start, start, Target::Word));
    let mut copy = text::copy();
    ui.operate(&mut black_box(&mut copy));
    assert!(matches!(copy.finish(), Outcome::Some(word) if word == "Clip"));
    assert!(!checked(&mut ui));
    ui.tap_key(Named::Space);
    assert!(checked(&mut ui));
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
}

#[cfg(any(feature = "wgpu", feature = "vello-cpu"))]
#[test]
fn inline_label_matches_ordinary_text_with_default_and_compact_typography() {
    use iced_core::Pixels;
    use iced_core::widget::Id;
    use iced_widget::{Widget, container, text};

    const LABEL: &str = "Clip content";
    for compact in [false, true] {
        let mut reference = text(LABEL);
        if compact {
            reference = reference.size(12).line_height(Pixels(16.));
        }
        let ordinary: Element<'_, bool, Theme, Renderer> = reference.boxed();
        let expected = simulator(ordinary).find(LABEL).unwrap().bounds();
        assert!(expected.width > 0.);
        if compact {
            assert_eq!(expected.height, 16.);
        }
        for selected in [false, true] {
            let mut checkbox = checkbox()
                .label(LABEL)
                .checked(selected)
                .on_checked_change(|value| value);
            if compact {
                checkbox = checkbox.text_size(12).line_height(Pixels(16.));
            }
            let element: Element<'_, bool, Theme, Renderer> = checkbox.into();
            let mut ui = simulator(container(element).center_y(24).id("row").boxed());
            let row = ui.find(Id::new("row")).unwrap().bounds();
            let label = ui.find(LABEL).unwrap().bounds();
            assert_eq!(
                label.size(),
                expected.size(),
                "compact={compact}, selected={selected}"
            );
            assert_eq!(row.height, 24.);
            assert!((label.center_y() - row.center_y()).abs() < 1.);
            assert_eq!(focus_count(&mut ui).total, 1);
            ui.click(LABEL).unwrap();
            assert_eq!(ui.drain().collect::<Vec<_>>(), [!selected]);
        }
    }
}

#[test]
fn space_activates_on_release_once_and_enter_does_not_toggle() {
    let mut ui = simulator(element());
    advance(&mut ui, false);
    assert_eq!(focus_count(&mut ui).focused, Some(0));
    ui.simulate([simulator::press_key(Named::Space, None)]);
    ui.simulate([simulator::press_key(Named::Space, None)]);
    assert!(!checked(&mut ui));
    assert_eq!(ui.drain().count(), 0);
    ui.simulate([simulator::release_key(Named::Space)]);
    assert!(checked(&mut ui));
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
    ui.tap_key(Named::Enter);
    assert!(checked(&mut ui));
    assert_eq!(ui.drain().count(), 0);
    ui.tap_key(Named::Space);
    assert!(!checked(&mut ui));
    assert_eq!(ui.drain().collect::<Vec<_>>(), [false]);
}

#[test]
fn focus_traversal_skips_disabled_and_cancels_space_when_focus_moves() {
    let contents: Element<'static, bool, Theme, Renderer> = column![
        checkbox().on_checked_change(|value| value),
        checkbox().disabled(true),
        checkbox().on_checked_change(|value| value),
    ]
    .spacing(20)
    .into();
    let mut ui = simulator(contents);
    assert_eq!(focus_count(&mut ui).total, 2);
    advance(&mut ui, false);
    ui.simulate([simulator::press_key(Named::Space, None)]);
    advance(&mut ui, false);
    assert_eq!(focus_count(&mut ui).focused, Some(1));
    ui.simulate([simulator::release_key(Named::Space)]);
    assert_eq!(ui.drain().count(), 0);
    advance(&mut ui, true);
    assert_eq!(focus_count(&mut ui).focused, Some(0));
    ui.tap_key(Named::Space);
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
}

#[test]
fn expanded_pointer_target_toggles_and_dragging_out_cancels() {
    let mut ui = simulator(element());
    // Outside the 16px paint box, inside Nova's 12px horizontal extension.
    ui.point_at(Point::new(25.0, 8.0));
    ui.simulate(simulator::click());
    assert!(checked(&mut ui));
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
    ui.simulate(simulator::click());
    assert!(!checked(&mut ui));
    ui.simulate(simulator::click());
    assert!(checked(&mut ui));
    assert_eq!(ui.drain().collect::<Vec<_>>(), [false, true]);
    ui.point_at(Point::new(8.0, 8.0));
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(Point::new(60.0, 8.0));
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    assert!(checked(&mut ui));
    assert_eq!(ui.drain().count(), 0);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(Point::new(8.0, 8.0));
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn disabled_inheritance_blocks_pointer_keyboard_and_label_activation() {
    for state in [
        checkbox::State::Unchecked,
        checkbox::State::Checked,
        checkbox::State::Indeterminate,
    ] {
        let mut slot: Slot<'static, bool> = field(
            checkbox()
                .default_checked(state)
                .on_checked_change(|value| value),
        )
        .orientation(field::Horizontal)
        .label(Slot::from(Space::new().width(100).height(20)))
        .into();
        slot.inherit(&Inherited::default().with_disabled(true));
        let mut ui = simulator(slot.compose());
        let initial = checked(&mut ui);
        assert_eq!(focus_count(&mut ui).total, 0);
        for x in [8.0, 60.0] {
            ui.point_at(Point::new(x, 10.0));
            ui.simulate(simulator::click());
        }
        advance(&mut ui, false);
        ui.tap_key(Named::Space);
        assert_eq!(checked(&mut ui), initial);
        assert_eq!(ui.drain().count(), 0);
    }
}

#[test]
fn disabled_rebuild_cancels_a_pointer_press_and_removes_focus() {
    let mut ui = simulator(element());
    ui.point_at(Point::new(8.0, 8.0));
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui = ui.rebuild(Element::from(
        checkbox().disabled(true).on_checked_change(|value| value),
    ));
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    assert!(!checked(&mut ui));
    assert_eq!(focus_count(&mut ui).total, 0);
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn uncontrolled_rebuild_retains_value_and_controlled_keyboard_only_publishes() {
    let mut ui = simulator(element());
    advance(&mut ui, false);
    ui.tap_key(Named::Space);
    ui = ui.rebuild(element());
    assert!(checked(&mut ui));
    ui = ui.rebuild(Element::from(
        checkbox().checked(false).on_checked_change(|value| value),
    ));
    assert!(!checked(&mut ui));
    ui.drain().for_each(drop);
    ui.tap_key(Named::Space);
    ui.tap_key(Named::Space);
    assert!(!checked(&mut ui));
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true, true]);
}

#[test]
fn indeterminate_commits_to_checked_then_unchecked() {
    let mut ui = simulator(Element::from(
        checkbox()
            .default_checked(checkbox::State::Indeterminate)
            .on_checked_change(|value| value),
    ));
    advance(&mut ui, false);
    ui.tap_key(Named::Space);
    assert!(checked(&mut ui));
    ui.tap_key(Named::Space);
    assert!(!checked(&mut ui));
    assert_eq!(ui.drain().collect::<Vec<_>>(), [true, false]);
}

#[test]
fn card_and_label_overlap_with_expanded_target_never_toggle_twice() {
    for card in [false, true] {
        let mut content = field(checkbox().on_checked_change(|value| value))
            .orientation(field::Horizontal)
            .label(Slot::from(Space::new().width(100).height(20)));
        if card {
            content = content.card();
        }
        let mut ui = simulator(Element::from(content));
        ui.point_at(Point::new(
            if card { 35.0 } else { 25.0 },
            if card { 20.0 } else { 10.0 },
        ));
        ui.simulate(simulator::click());
        ui.tap_key(Named::Enter); // Flush a linked label's deferred activation.
        assert!(checked(&mut ui));
        assert_eq!(ui.drain().collect::<Vec<_>>(), [true]);
        if card {
            // Interior whitespace beyond the primitive's expanded target.
            ui.point_at(Point::new(120.0, 20.0));
            ui.simulate(simulator::click());
            ui.tap_key(Named::Enter);
            assert!(!checked(&mut ui));
            assert_eq!(ui.drain().collect::<Vec<_>>(), [false]);
        }
    }
}

#[test]
fn complete_card_padding_activates_once_and_inherits_disabled() {
    for disabled in [false, true] {
        let mut ui = simulator(Element::from(
            field(checkbox().on_checked_change(|value| value))
                .orientation(field::Horizontal)
                .label(Slot::from(Space::new().width(100).height(20)))
                .card()
                .disabled(disabled),
        ));
        // Far right/bottom border, beyond both the inner content and expanded box.
        ui.point_at(Point::new(145.0, 41.0));
        ui.simulate(simulator::click());
        ui.tap_key(Named::Enter);
        assert_eq!(checked(&mut ui), !disabled);
        assert_eq!(
            ui.drain().collect::<Vec<_>>(),
            if disabled { vec![] } else { vec![true] }
        );
        ui.simulate(simulator::click());
        ui.tap_key(Named::Enter);
        assert!(!checked(&mut ui));
        assert_eq!(
            ui.drain().collect::<Vec<_>>(),
            if disabled { vec![] } else { vec![false] }
        );
    }
}
