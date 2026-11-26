use browser::component::tabs::TabsPage;
use iced::keyboard::key::Named;
use iced::{Element, Point, Size};
use iced_test::core::widget::operation::{Operation, Outcome, focusable};
use iced_test::{Simulator, simulator};
use icedcn::Theme;
use icedcn::widget::{button, column, tabs};

type Ui<'a> = Simulator<'a, String, Theme, iced::Renderer>;

fn view(selected: Option<&str>, vertical: bool) -> Element<'_, String, Theme> {
    let mut strip = tabs(|t| {
        column![
            t.list([
                t.trigger("one", "First entry"),
                t.trigger("two", "Unavailable")
                    .focus_border(1., |theme| theme.neutral.ring)
                    .disabled(true),
                t.trigger("three", "Third entry")
                    .focus_border(1., |theme| theme.neutral.ring),
                t.trigger("four", "Fourth entry"),
            ]),
            t.content("one", "First panel"),
            t.content("three", "Third panel"),
            t.content("four", "Fourth panel"),
        ]
        .into()
    })
    .default_value("three")
    .on_value_change(|v| v);
    if vertical {
        strip = strip.orientation(tabs::Orientation::Vertical);
    }
    if let Some(value) = selected {
        strip = strip.value(value);
    }
    column![strip, button("After group").on_press("after".to_owned())].into()
}

fn next(ui: &mut Ui<'_>) {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
}

#[test]
fn only_the_selected_tab_is_in_page_traversal_and_arrows_skip_disabled() {
    let mut ui = Simulator::with_size(Default::default(), Size::new(600., 300.), view(None, false));
    next(&mut ui);
    ui.tap_key(Named::ArrowLeft);
    ui.find("First panel").unwrap();
    ui.tap_key(Named::ArrowLeft);
    ui.find("Fourth panel").unwrap();
    ui.tap_key(Named::Home);
    ui.find("First panel").unwrap();
    ui.tap_key(Named::End);
    ui.find("Fourth panel").unwrap();
    assert_eq!(
        ui.drain().collect::<Vec<_>>(),
        ["one", "four", "one", "four"]
    );
    next(&mut ui);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().collect::<Vec<_>>(), ["after"]);
}

#[test]
fn vertical_uses_up_down_and_controlled_selection_waits_for_the_app() {
    let mut ui = Simulator::with_size(
        Default::default(),
        Size::new(400., 300.),
        view(Some("one"), true),
    );
    ui.click("Unavailable").unwrap();
    assert_eq!(ui.drain().count(), 0);
    next(&mut ui);
    ui.tap_key(Named::ArrowRight);
    assert_eq!(ui.drain().count(), 0);
    ui.tap_key(Named::ArrowDown);
    ui.find("First panel").unwrap();
    assert_eq!(ui.drain().collect::<Vec<_>>(), ["three"]);
    let mut ui = ui.rebuild(view(Some("three"), true));
    ui.find("Third panel").unwrap();
    ui.tap_key(Named::ArrowUp);
    assert_eq!(ui.drain().collect::<Vec<_>>(), ["one"]);
}

#[test]
fn keyboard_reveals_both_ends_of_an_overflowing_strip() {
    let mut ui = Simulator::with_size(Default::default(), Size::new(150., 300.), view(None, false));
    next(&mut ui);
    ui.draw(&Theme::light());
    let selected = ui.find("Third entry").unwrap();
    assert!((selected.visible_bounds().unwrap().width - selected.bounds().width).abs() < 1.);

    ui.tap_key(Named::End);
    ui.draw(&Theme::light());
    let t = ui.find("Fourth entry").unwrap();
    let b = t.bounds();
    let r = t.visible_bounds().expect("last tab is visible");
    assert!((r.width - b.width).abs() < 1., "{r:?} vs {b:?}");
    ui.tap_key(Named::Home);
    ui.draw(&Theme::light());
    let t = ui.find("First entry").unwrap();
    assert!((t.visible_bounds().unwrap().width - t.bounds().width).abs() < 1.);
}

#[test]
fn the_registered_page_switches_all_four_panels_and_vertical_rows_are_full_width() {
    let page = TabsPage::new(0);
    let mut ui = Simulator::with_size(
        Default::default(),
        Size::new(290.2, 384.),
        page.preview("demo"),
    );
    for (tab, body) in [
        ("Analytics", "Page views are up 25% compared to last month."),
        (
            "Reports",
            "You have 5 reports ready and available to export.",
        ),
        ("Settings", "Configure notifications, security, and themes."),
        (
            "Overview",
            "You have 12 active projects and 3 pending tasks.",
        ),
    ] {
        ui.click(tab).unwrap();
        ui.find(body).unwrap();
    }
    let mut ui = Simulator::with_size(Default::default(), Size::new(280., 288.), view(None, true));
    let first = ui.find("First entry").unwrap().bounds();
    let right = ui.find("Unavailable").unwrap().bounds();
    ui.point_at(Point::new(right.x + right.width, first.center_y()));
    ui.simulate(simulator::click());
    assert_eq!(ui.drain().collect::<Vec<_>>(), ["one"]);
}
