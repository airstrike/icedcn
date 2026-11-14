//! Keyboard navigation through compound controls and modal focus scopes.
use iced_core::keyboard::key::Named;
use iced_core::time::Instant;
use iced_core::widget::operation::{self, Operation, Outcome, focusable};
use iced_core::{Element, Event, Point, window};
use iced_test::simulator::{self, simulator};
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_widget::{accordion, button, column, dialog};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Open(Vec<String>),
    Pressed,
}
type Simulator<'a> = iced_test::Simulator<'a, Message, Theme, Renderer>;

fn focus(ui: &mut Simulator<'_>) -> focusable::Count {
    let mut count = focusable::count();
    ui.operate(&mut operation::black_box(&mut count));
    let Outcome::Some(count) = count.finish() else {
        panic!("focus count")
    };
    count
}

fn frame(ui: &mut Simulator<'_>) {
    let _ = ui.simulate([Event::Window(
        window::Event::RedrawRequested(Instant::now()),
    )]);
}

#[test]
fn accordion_header_keeps_its_height_below_a_scrollable_viewport() {
    use iced_core::mouse::ScrollDelta;
    use iced_core::widget::Id;
    use iced_widget::Space;
    use icedcn_widget::{container, scrollable};

    let id = Id::unique();
    let label = container(Space::new().width(80).height(20)).id(id.clone());
    let disclosure = accordion([accordion::item("shipping", label)]).on_value_change(Message::Open);
    let content = column![Space::new().height(500), disclosure];
    let mut ui = simulator(Element::from(scrollable::scrollable(content).height(200)));

    ui.point_at(Point::new(25., 50.));
    ui.scroll(ScrollDelta::Pixels { x: 0., y: -600. });
    frame(&mut ui);
    let visible = ui
        .find(id)
        .expect("header is mounted")
        .visible_bounds()
        .expect("header scrolled into view");
    assert_eq!(visible.height, 20.);
    ui.point_at(visible.center());
    ui.simulate(simulator::click());
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(vec!["shipping".into()])]
    );
}

#[test]
fn accordion_fill_trigger_uses_its_intrinsic_height_below_a_scrollable_viewport() {
    use iced_core::Length::Fill;
    use iced_core::widget::Id;
    use iced_widget::Space;
    use icedcn_widget::{container, scrollable};

    let id = Id::unique();
    let trigger = container(Space::new().width(80).height(20))
        .height(Fill)
        .id(id.clone());
    let disclosure = accordion([accordion::item("custom", trigger)]);
    let content = column![Space::new().height(500), disclosure];
    let mut ui: Simulator<'_> =
        simulator(Element::from(scrollable::scrollable(content).height(200)));
    let bounds = ui
        .find(id)
        .expect("public custom trigger is mounted")
        .bounds();
    assert_eq!(bounds.height, 20.);
}

#[test]
fn accordion_arrows_skip_disabled_headers_and_activate_the_selected_item() {
    let content: Element<'_, Message, Theme, Renderer> = accordion([
        accordion::item("one", "One"),
        accordion::item("two", "Two").disabled(true),
        accordion::item("three", "Three").content(button("Child")),
    ])
    .on_value_change(Message::Open)
    .into();
    let mut ui = simulator(content);
    assert_eq!(
        focus(&mut ui).total,
        2,
        "closed panels are outside the focus order"
    );
    let _ = ui.simulate([simulator::press_key(Named::Tab, None)]);
    // The host's Tab handler uses the iced operation; simulate its two passes.
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut step) = next.finish() {
        ui.operate(step.as_mut());
    }
    ui.tap_key(Named::ArrowDown);
    assert_eq!(focus(&mut ui).focused, Some(1));
    ui.tap_key(Named::Space);
    assert_eq!(focus(&mut ui).total, 3);
    ui.tap_key(Named::Home);
    assert_eq!(focus(&mut ui).focused, Some(0));
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(vec!["three".into()])]
    );
}

#[test]
fn dialog_focus_enters_wraps_and_returns_on_escape() {
    let body = || -> Element<'static, Message, Theme, Renderer> {
        dialog(
            |d| d.trigger(button("Open")),
            |_| column![button("One").on_press(Message::Pressed), button("Two")].into(),
        )
        .show_close_button(false)
        .into()
    };
    let mut ui = simulator(body());
    ui.point_at(Point::new(10.0, 10.0));
    let _ = ui.simulate(simulator::click());
    frame(&mut ui);
    assert_eq!(
        focus(&mut ui),
        focusable::Count {
            focused: Some(0),
            total: 2
        }
    );
    ui.tap_key(Named::Tab);
    assert_eq!(focus(&mut ui).focused, Some(1));
    ui.tap_key(Named::Tab);
    assert_eq!(focus(&mut ui).focused, Some(0));
    ui.tap_key(Named::Enter);
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(body());
    frame(&mut ui);
    assert_eq!(
        focus(&mut ui),
        focusable::Count {
            focused: Some(0),
            total: 1
        }
    );
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [Message::Pressed]);
}

#[test]
fn tabs_skip_disabled_triggers_and_activate_on_vertical_arrows() {
    use icedcn_widget::tabs;
    let element: Element<'static, Message, Theme, Renderer> = tabs(|t| {
        t.list([
            t.trigger("first", "First"),
            t.trigger("second", "Second").disabled(true),
            t.trigger("third", "Third"),
        ])
    })
    .orientation(tabs::Orientation::Vertical)
    .line(true)
    .default_value("first")
    .on_value_change(|v| Message::Open(vec![v]))
    .into();
    let mut ui = simulator(element);
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut step) = next.finish() {
        ui.operate(step.as_mut());
    }
    // Global Tab traversal exposes one entry for the entire tab list.
    assert_eq!(
        focus(&mut ui),
        focusable::Count {
            focused: Some(0),
            total: 1
        }
    );
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowDown);
    assert_eq!(focus(&mut ui).focused, Some(0));
    ui.tap_key(Named::Home);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [
            Message::Open(vec!["third".into()]),
            Message::Open(vec!["first".into()])
        ]
    );
}

#[test]
fn combobox_filters_custom_values_and_selects_the_highlight_from_the_keyboard() {
    use icedcn_widget::combobox;
    let widget: Element<'_, Message, Theme, Renderer> = combobox::options([
        combobox::item("us", "United States").group("America"),
        combobox::item("gb", "United Kingdom").description("Europe (gb)"),
    ])
    .auto_highlight(true)
    .on_value_change(|value| Message::Open(vec![value]))
    .into();
    let mut ui = simulator(widget);
    ui.point_at(Point::new(25.0, 16.0));
    let _ = ui.simulate(simulator::click());
    frame(&mut ui);
    ui.typewrite("United");
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(vec!["gb".into()])]
    );
}

#[test]
fn command_search_skips_disabled_results_and_runs_the_highlighted_action() {
    use icedcn_widget::command;
    let widget: Element<'_, Message, Theme, Renderer> = command([
        command::item("Calendar").disabled(true),
        command::item("Calculate").on_select(Message::Pressed),
        command::item("Profile").on_select(Message::Open(vec![])),
    ])
    .into();
    let mut ui = simulator(widget);
    ui.point_at(Point::new(25.0, 16.0));
    let _ = ui.simulate(simulator::click());
    frame(&mut ui);
    ui.typewrite("Cal");
    ui.tap_key(Named::Enter);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [Message::Pressed]);
}

fn focus_next(ui: &mut Simulator<'_>) {
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut step) = next.finish() {
        ui.operate(step.as_mut());
    }
}

#[test]
fn tooltip_opens_on_keyboard_focus_and_escape_suppresses_it_until_refocus() {
    use icedcn_widget::tooltip;
    let tip: Element<'_, Message, Theme, Renderer> = tooltip(button("Help"), "Hint")
        .on_open_change(|open| Message::Open(vec![open.to_string()]))
        .into();
    let mut ui = simulator(tip);
    focus_next(&mut ui);
    frame(&mut ui);
    ui.tap_key(Named::Escape);
    frame(&mut ui);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [
            Message::Open(vec!["true".into()]),
            Message::Open(vec!["false".into()])
        ]
    );
}

#[test]
fn submenu_arrow_keys_enter_the_first_item_and_return_to_the_trigger() {
    use icedcn_widget::menu;
    let body: Element<'_, Message, Theme, Renderer> = menu::sub(
        "More",
        [menu::item("Choose").on_select(Message::Pressed).into()],
    )
    .into();
    let mut ui = simulator(body);
    focus_next(&mut ui);
    ui.tap_key(Named::ArrowRight);
    frame(&mut ui);
    ui.tap_key(Named::ArrowLeft);
    frame(&mut ui);
    assert_eq!(focus(&mut ui).total, 1);
    ui.tap_key(Named::ArrowRight);
    frame(&mut ui);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [Message::Pressed]);
}

#[test]
fn context_menu_stationary_touch_opens_but_vertical_scrolling_does_not() {
    use iced_core::{time::Duration, touch};
    use icedcn_widget::{container, context_menu, menu};
    let body: Element<'_, Message, Theme, Renderer> = context_menu(
        |_| container("Area").width(200).height(100).into(),
        |_| menu::group([menu::item("Inspect").into()]),
    )
    .on_open_change(|open| Message::Open(vec![open.to_string()]))
    .into();
    let mut ui = simulator(body);
    let position = Point::new(12., 12.);
    ui.point_at(position);
    let id = touch::Finger(0);
    let down = Event::Touch(touch::Event::FingerPressed { id, position });
    let later = || {
        Event::Window(window::Event::RedrawRequested(
            Instant::now() + Duration::from_millis(710),
        ))
    };
    let _ = ui.simulate([
        down.clone(),
        Event::Touch(touch::Event::FingerMoved {
            id,
            position: Point::new(12., 48.),
        }),
        later(),
        Event::Touch(touch::Event::FingerLifted { id, position }),
    ]);
    let _ = ui.simulate([down, later()]);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(vec!["true".into()])]
    );
}

#[test]
fn drawer_outward_swipe_closes_on_the_selected_edge() {
    use iced_core::touch;
    use icedcn_widget::{container, drawer};
    let body: Element<'_, Message, Theme, Renderer> = drawer(
        |d| d.trigger(button("Open")),
        |_| container("Panel").width(200).height(200).into(),
    )
    .default_open(true)
    .side(drawer::Side::Left)
    .on_open_change(|open| Message::Open(vec![open.to_string()]))
    .into();
    let mut ui = simulator(body);
    let start = Point::new(160., 120.);
    ui.point_at(start);
    let id = touch::Finger(1);
    let _ = ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            id,
            position: start,
        }),
        Event::Touch(touch::Event::FingerLifted {
            id,
            position: Point::new(24., 130.),
        }),
    ]);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(vec!["false".into()])]
    );
}

#[test]
fn mobile_sidebar_opens_in_a_sheet_without_changing_desktop_preference() {
    use iced_core::Size;
    use icedcn_widget::{row, sidebar};
    let body = || -> Element<'static, Message, Theme, Renderer> {
        sidebar::provider(|s| row![s.sidebar(button("Workspace")), sidebar::trigger()].into())
            .on_open_change(|_| Message::Pressed)
            .into()
    };
    let mut ui = Simulator::with_size(Default::default(), Size::new(390., 800.), body());
    frame(&mut ui);
    ui = ui.rebuild(body());
    frame(&mut ui);
    assert_eq!(focus(&mut ui).total, 1);
    ui.point_at(Point::new(12., 14.));
    let _ = ui.simulate(simulator::click());
    frame(&mut ui);
    assert_eq!(focus(&mut ui).focused, Some(0));
    ui.tap_key(Named::Escape);
    ui = ui.rebuild(body());
    frame(&mut ui);
    assert_eq!(focus(&mut ui).total, 1);
    assert_eq!(ui.into_messages().count(), 0);
}

#[test]
fn dropdown_keyboard_focus_reaches_submenus_and_skips_disabled_items() {
    use icedcn_widget::{dropdown_menu, menu};
    let body: Element<'_, Message, Theme, Renderer> = dropdown_menu(
        |m| m.trigger(button("Menu")),
        |_| {
            menu::group([
                menu::item("First").into(),
                menu::item("Disabled").disabled(true).into(),
                menu::sub(
                    "More",
                    [menu::item("Choose").on_select(Message::Pressed).into()],
                ),
            ])
        },
    )
    .into();
    let mut ui = simulator(body);
    focus_next(&mut ui);
    ui.tap_key(Named::Enter);
    frame(&mut ui);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::ArrowRight);
    frame(&mut ui);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), [Message::Pressed]);
}

#[test]
fn combobox_popup_moves_focus_to_the_search_input() {
    use icedcn_widget::combobox;
    let body: Element<'_, Message, Theme, Renderer> = combobox(["Apple", "Apricot", "Pear"])
        .popup(true)
        .auto_highlight(true)
        .on_value_change(|s| Message::Open(vec![s]))
        .into();
    let mut ui = simulator(body);
    ui.point_at(Point::new(12., 16.));
    let _ = ui.simulate(simulator::click());
    frame(&mut ui);
    ui.typewrite("Ap");
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        [Message::Open(vec!["Apricot".into()])]
    );
}

#[test]
fn responsive_drawer_uses_window_width_and_preserves_open_state() {
    use iced_core::Size;
    use icedcn_widget::drawer;
    let view = || -> Element<'static, Message, Theme, Renderer> {
        drawer(
            |d| d.trigger(button("Open")),
            |d| {
                if d.is_mobile() {
                    column![
                        button("Save").on_press(Message::Open(vec!["mobile".into()])),
                        d.close(button("Cancel"))
                    ]
                    .into()
                } else {
                    button("Save")
                        .on_press(Message::Open(vec!["desktop".into()]))
                        .into()
                }
            },
        )
        .desktop_dialog(425.)
        .default_open(true)
        .into()
    };
    for width in [390., 1024.] {
        let mut ui = Simulator::with_size(Default::default(), Size::new(width, 800.), view());
        frame(&mut ui);
        ui = ui.rebuild(view());
        frame(&mut ui);
        assert_eq!(focus(&mut ui).total, 2);
        // Mobile gets form + cancel; desktop gets form + corner close.
        assert_eq!(focus(&mut ui).focused, Some(0));
        ui.tap_key(Named::Enter);
        let mode = if width < 768. { "mobile" } else { "desktop" };
        assert_eq!(
            ui.into_messages().collect::<Vec<_>>(),
            [Message::Open(vec![mode.into()])]
        );
    }
}
