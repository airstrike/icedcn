//! Keyboard and placement regressions for the Select examples.
use browser::Message;
use browser::component::select::SelectPage;
use browser::page::Page;
use iced::keyboard::key::Named;
use iced::{Point, Size};
use iced_test::{Simulator, simulator};
use iced_test::core::Settings;
use icedcn::Theme;

type Preview<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

/// Component events from overlays invalidate the application view.
fn rebuild<'a>(ui: Preview<'a>, page: &'a dyn Page, id: &str) -> Preview<'a> {
    let mut ui = ui.rebuild(page.preview(id));
    ui.draw(&Theme::light());
    ui
}

#[test]
fn keyboard_selection_updates_the_trigger_and_escape_keeps_its_value() {
    let page = SelectPage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(400., 288.),
        page.preview("demo"),
    );
    ui.click("Select a fruit").unwrap();
    ui.draw(&Theme::light());
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    let mut ui = rebuild(ui, page.as_ref(), "demo");
    ui.find("Banana").unwrap();
    assert!(ui.find("Apple").is_err());
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::End);
    ui.tap_key(Named::Escape);
    let mut ui = rebuild(ui, page.as_ref(), "demo");
    ui.find("Banana").unwrap();
    assert!(ui.find("Pineapple").is_err());
    ui.click("Banana").unwrap();
    ui.draw(&Theme::light());
    ui.typewrite("blue");
    ui.tap_key(Named::Enter);
    let mut ui = rebuild(ui, page.as_ref(), "demo");
    ui.find("Blueberry").unwrap();
    assert!(ui.find("Fruits").is_err());
    ui.typewrite("pine");
    ui.find("Pineapple").unwrap();
    assert!(ui.find("Fruits").is_err());
}

#[test]
fn end_selects_the_last_timezone_in_a_short_viewport() {
    let page = SelectPage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(340., 270.),
        page.preview("scrollable"),
    );
    ui.click("Select a timezone").unwrap();
    ui.draw(&Theme::light());
    ui.tap_key(Named::End);
    ui.tap_key(Named::Enter);
    let mut ui = rebuild(ui, page.as_ref(), "scrollable");
    ui.find("Chile Standard Time").unwrap();
    assert!(ui.find("Select a timezone").is_err());
    ui.click("Chile Standard Time").unwrap();
    ui.draw(&Theme::light());
    ui.tap_key(Named::Home);
    ui.tap_key(Named::Enter);
    let mut ui = rebuild(ui, page.as_ref(), "scrollable");
    ui.find("Eastern Standard Time").unwrap();
}

#[test]
fn switch_moves_the_popup_below_the_trigger() {
    let page = SelectPage::new(0);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(340., 400.),
        page.preview("align-item"),
    );
    let trigger = ui.click("Banana").unwrap();
    ui.draw(&Theme::light());
    let apple = ui.find("Apple").unwrap();
    assert!(apple.bounds().y < trigger.bounds().y);
    ui.tap_key(Named::Escape);
    let mut ui = rebuild(ui, page.as_ref(), "align-item");
    ui.click("Align Item").unwrap();
    ui.draw(&Theme::light());
    ui.click("Banana").unwrap();
    ui.draw(&Theme::light());
    let apple = ui.find("Apple").unwrap();
    assert!(apple.bounds().y >= trigger.bounds().y + trigger.bounds().height);
    ui.point_at(Point::new(339., 390.));
    ui.simulate(simulator::click());
    let mut ui = rebuild(ui, page.as_ref(), "align-item");
    assert!(ui.find("Apple").is_err());
}
